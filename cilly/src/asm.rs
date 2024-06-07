use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call_site::CallSite,
    cil_node::CILNode,
    cil_root::CILRoot,
    method::{Method, MethodType},
    static_field_desc::StaticFieldDescriptor,
    type_def::TypeDef,
    FnSig, IString, Type,
};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
/// Data representing a reference to an external assembly.
pub struct AssemblyExternRef {
    /// A tuple describing the referenced assebmly.
    /// Tuple contains:
    /// (Major Version, Minor Version, Revision number, Build number)
    /// In that order.
    version: (u16, u16, u16, u16),
}
impl AssemblyExternRef {
    /// Returns the version information of this assembly.
    #[must_use]
    pub fn version(&self) -> (u16, u16, u16, u16) {
        self.version
    }
}
pub type ExternFnDef = (IString, FnSig, bool);
#[derive(Serialize, Deserialize, Debug)]
/// Representation of a .NET assembly.
pub struct Assembly {
    /// List of types desined within the assembly.
    types: HashMap<IString, TypeDef>,
    /// List of functions defined within this assembly.
    functions: HashMap<CallSite, Method>,
    /// Callsite representing the entrypoint of this assebmly if any present.
    entrypoint: Option<CallSite>,
    /// List of references to external assemblies
    extern_refs: HashMap<IString, AssemblyExternRef>,
    extern_fns: HashMap<ExternFnDef, IString>,
    /// List of all static fields within the assembly
    static_fields: HashMap<IString, Type>,
}
impl Assembly {
    pub fn call_graph(&self)->String{
        let mut res = format!("digraph mygraph {{\nfontname=\"Helvetica,Arial,sans-serif\"\nnode [fontname=\"Helvetica,Arial,sans-serif\"]
edge [fontname=\"Helvetica,Arial,sans-serif\"]\nnode [shape=box];\n");
        for (_,function) in self.functions(){
            let name = function.name();
            let calls:std::collections::HashSet<_> = function.calls().iter().filter_map(|site|if site.class().is_none(){Some(site.name())}else{None}).collect();
            for called in calls{
                res.push_str(&format!("\"{name}\"->\"{called}\"\n"))
            }
        }
        res.push('}');
        res
    }
    pub fn sizeof_tpedef(&self,tpe:&crate::DotnetTypeRef)->u64{
        assert!(tpe.asm().is_none());
        self.types.get(tpe.name_path()).unwrap()
        .explict_size().unwrap()
    }
    pub fn save_tmp<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(&postcard::to_stdvec(&self).unwrap())
    }
    /// Returns iterator over all global fields
    pub fn globals(&self) -> impl Iterator<Item = (&IString, &Type)> {
        self.static_fields.iter()
    }
    /// Returns the `.cctor` function used to initialize static data
    #[must_use]
    pub fn cctor(&self) -> Option<&Method> {
        self.functions.get(&CallSite::new(
            None,
            ".cctor".into(),
            FnSig::new(&[], Type::Void),
            true,
        ))
    }
    /// Returns the external assembly reference
    #[must_use]
    pub fn extern_refs(&self) -> &HashMap<IString, AssemblyExternRef> {
        &self.extern_refs
    }
    /// Creates a new, empty assembly.
    #[must_use]
    pub fn empty() -> Self {
        let mut res = Self {
            types: HashMap::new(),
            functions: HashMap::new(),
            entrypoint: None,
            extern_refs: HashMap::new(),
            static_fields: HashMap::new(),
            extern_fns: HashMap::new(),
        };
        let dotnet_ver = AssemblyExternRef {
            version: (6, 12, 0, 0),
        };
        res.extern_refs.insert("System.Runtime".into(), dotnet_ver);
        //res.extern_refs.insert("mscorlib".into(),dotnet_ver);
        res.extern_refs
            .insert("System.Runtime.InteropServices".into(), dotnet_ver);
        // Needed to get C-Mode to work
        res.add_cctor();
        res
    }
    /// Joins 2 assemblies together.
    #[must_use]
    pub fn join(self, other: Self) -> Self {
        let static_initializer = link_static_initializers(self.cctor(), other.cctor());
        let mut types = self.types;
        types.extend(other.types);
        let mut functions = self.functions;
        functions.extend(other.functions);
        if let Some(static_initializer) = static_initializer {
            functions.insert(static_initializer.call_site(), static_initializer);
        }
        let entrypoint = self.entrypoint.or(other.entrypoint);
        let mut extern_refs = self.extern_refs;
        let mut static_fields = self.static_fields;
        let mut extern_fns = self.extern_fns;
        static_fields.extend(other.static_fields);
        extern_refs.extend(other.extern_refs);
        extern_fns.extend(other.extern_fns);
        Self {
            types,
            functions,
            entrypoint,
            extern_refs,
            extern_fns,
            static_fields,
        }
    }
    /// Gets the typdefef at path `path`.
    #[must_use]
    pub fn get_typedef_by_path(&self, path: &str) -> Option<&TypeDef> {
        if path.contains('/') {
            let mut path_iter = path.split('/');
            let path_first = path_iter.next().unwrap();
            let mut td = self.get_typedef_by_path(path_first)?;
            // FIXME: this loop is messy.
            for part in path_iter {
                let old = td;
                for inner in td.inner_types() {
                    if inner.name() == part {
                        td = inner;
                        break;
                    }
                }
                if td == old {
                    return None;
                }
            }
            return Some(td);
        }
        self.types()
            .find(|&tpe| tpe.0.as_ref() == path)
            .map(|t| t.1)
    }

    /// Adds a global static field named *name* of type *tpe*
    pub fn add_static(&mut self, tpe: Type, name: &str) {
        self.static_fields.insert(name.into(), tpe);
    }
    pub fn add_cctor(&mut self) -> &mut Method {
        self.functions
            .entry(CallSite::new(
                None,
                ".cctor".into(),
                FnSig::new(&[], Type::Void),
                true,
            ))
            .or_insert_with(|| {
                Method::new(
                    AccessModifer::Public,
                    MethodType::Static,
                    FnSig::new(&[], Type::Void),
                    ".cctor",
                    vec![
                        (None, Type::Ptr(Type::U8.into())),
                        (None, Type::Ptr(Type::U8.into())),
                    ],
                    vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
                    vec![],
                )
            })
    }

    /// Returns true if assembly contains function named `name`
    #[must_use]
    pub fn contains_fn_named(&self, name: &str) -> bool {
        //FIXME:This is inefficient.
        self.methods().any(|m| m.name() == name)
    }
    /// Returns true if assembly contains function named `name`
    #[must_use]
    pub fn contains_fn(&self, site: &CallSite) -> bool {
        self.functions.contains_key(site)
    }
    /// Adds a method to the assebmly.
    pub fn add_method(&mut self, mut method: Method) {
        method.allocate_temporaries();
        method.allocate_temporaries();
        //method.ensure_valid();

        self.functions.insert(method.call_site(), method);
    }
    /// Returns the list of all calls within the method. Calls may repeat.
    #[must_use]
    pub fn call_sites(&self) -> Vec<&CallSite> {
        self.methods().flat_map(Method::calls).collect()
    }
    pub fn remove_dead_statics(&mut self) {
        // Get the set of "alive" fields(fields referenced outside of the static initializer).
        let alive_fields: std::collections::HashSet<_> = self
            .methods()
            .filter(|method| method.name() != ".cctor")
            .flat_map(Method::sflds)
            .cloned()
            .collect();
        // Remove the definitions of all non-alive fields
        self.static_fields.retain(|name, tpe| {
            alive_fields.contains(&StaticFieldDescriptor::new(None, tpe.clone(), name.clone()))
        });
        // Remove their initializers from the cctor
        let Some(cctor) = self.cctor_mut() else {
            return;
        };
        for tree in cctor
            .blocks_mut()
            .iter_mut()
            .flat_map(BasicBlock::trees_mut)
        {
            if let CILRoot::SetStaticField { descr, value } = tree.root_mut() {
                // Assigement to a dead static, remove.
                if !alive_fields.contains(descr) {
                    debug_assert!(descr.name().contains('a'));
                    debug_assert!(matches!(value, CILNode::Call { site: _, args: _ }));
                    *tree = CILRoot::Nop.into();
                }
            }
        }
    }
    /// Returns an interator over all methods within the assembly.
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.values()
    }
    /// Returns an interator over all methods within the assembly.
    pub fn methods_mut(&mut self) -> impl Iterator<Item = &mut Method> {
        self.functions.values_mut()
    }
    /// Returns an iterator over all types witin the assembly.
    pub fn types(&self) -> impl Iterator<Item = (&IString, &TypeDef)> {
        self.types.iter()
    }
    /// Optimizes all the methods witin the assembly.
    pub fn opt(&mut self) {
        self.functions.iter_mut().for_each(|method| {
            let (site, method) = method;
            let mut method = method.clone();
            method.opt();
            //crate::opt::opt_method(&mut method, self);
        });
    }
    /// Adds a definition of a type to the assembly.
    pub fn add_typedef(&mut self, type_def: TypeDef) {
        self.types.insert(type_def.name().into(), type_def);
    }

    /// Sets the entrypoint of the assembly to the method behind `CallSite`.
    pub fn set_entrypoint(&mut self, entrypoint: &CallSite) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        let wrapper = crate::entrypoint::wrapper(entrypoint);
        self.entrypoint = Some(wrapper.call_site());
        self.add_method(wrapper);
    }

    #[must_use]
    pub fn extern_fns(&self) -> &HashMap<ExternFnDef, IString> {
        &self.extern_fns
    }

    pub fn add_extern_fn(&mut self, name: IString, sig: FnSig, lib: IString, preserve_errno: bool) {
        self.extern_fns.insert((name, sig, preserve_errno), lib);
    }
    fn get_exported_fn(&self) -> HashMap<CallSite, Method> {
        let mut externs = HashMap::new();
        if let Some(entrypoint) = &self.entrypoint {
            let method = self.functions.get(entrypoint).cloned().unwrap();
            externs.insert(entrypoint.clone(), method);
        }
        if let Some(cctor) = self.cctor() {
            externs.insert(
                CallSite::new(None, ".cctor".into(), FnSig::new(&[], Type::Void), true),
                cctor.clone(),
            );
        }
        for call in self
            .types()
            .flat_map(|(_, type_def)| type_def.methods())
            .flat_map(|method| (method.calls()))
        {
            if let Some(method) = self.functions.get(&call).cloned() {
                externs.insert(call.clone(), method);
            };
        }
        externs
    }
    pub fn eliminate_dead_fn(&mut self) {
        let mut alive: HashMap<CallSite, Method> = HashMap::new();
        let mut resurecting: HashMap<CallSite, Method> = HashMap::new();
        let mut to_resurect: HashMap<CallSite, Method> = self.get_exported_fn();
        while !to_resurect.is_empty() {
            alive.extend(resurecting.clone());
            resurecting.clear();
            resurecting.extend(to_resurect.clone());
            to_resurect.clear();
            for call in resurecting.iter().flat_map(|fnc| fnc.1.calls()) {
                if let Some(_class) = call.class() {
                    // TODO: if dead code elimination too agressive check this
                    // Methods reference by methods inside types are NOT tracked.
                    continue;
                }
                if alive.contains_key(&call) || resurecting.contains_key(&call) {
                    // Already alive, ignore!
                    continue;
                }
                if let Some(method) = self.functions.get(&call).cloned() {
                    to_resurect.insert(call.clone(), method);
                };
            }
        }
        alive.extend(resurecting);
        self.functions = alive;
    }
    pub fn eliminate_dead_code(&mut self) {
        if *DEAD_CODE_ELIMINATION {
            self.eliminate_dead_fn();
            self.remove_dead_statics();
            // Call eliminate_dead_fn again, to remove now-dead static initializers.
            self.eliminate_dead_fn();
        }

        //self.eliminate_dead_types();
    }
    pub fn eliminate_dead_types(&mut self) {
        let mut alive = HashMap::new();
        let mut resurected: HashMap<IString, _> = self
            .functions
            .values()
            .flat_map(Method::dotnet_types)
            .filter_map(|tpe| match tpe.asm() {
                Some(_) => None,
                None => Some(IString::from(tpe.name_path())),
            })
            .map(|name| (name.clone(), self.types.get(&name).unwrap().clone()))
            .collect();
        resurected.insert(
            "RustVoid".into(),
            self.types.get("RustVoid").cloned().unwrap(),
        );
        let mut to_resurect: HashMap<IString, _> = HashMap::new();
        while !resurected.is_empty() {
            for tpe in &resurected {
                alive.insert(tpe.0.clone(), tpe.1.clone());
                for (name, type_def) in tpe
                    .1
                    .all_types()
                    .filter_map(Type::dotnet_refs)
                    .filter_map(|tpe| match tpe.asm() {
                        Some(_) => None,
                        None => Some(IString::from(tpe.name_path())),
                    })
                    .filter_map(|name| name.split_once('\\').map(|(a, _)| a.into()))
                    //.map(|(a,b)|a.into())
                    .map(|name: IString| {
                        (
                            name.clone(),
                            self.types
                                .get(&name)
                                .unwrap_or_else(|| panic!("Can't find type {name:?}"))
                                .clone(),
                        )
                    })
                {
                    let name: IString = IString::from(name);
                    to_resurect.insert(name, type_def);
                }
            }
            resurected = to_resurect;
            to_resurect = HashMap::new();
        }
        self.types = alive;
    }

    pub fn cctor_mut(&mut self) -> Option<&mut Method> {
        self.functions.get_mut(&CallSite::new(
            None,
            ".cctor".into(),
            FnSig::new(&[], Type::Void),
            true,
        ))
    }

    pub fn static_fields_mut(&mut self) -> &mut HashMap<IString, Type> {
        &mut self.static_fields
    }

    pub fn functions(&self) -> &HashMap<CallSite, Method> {
        &self.functions
    }
}
use lazy_static::*;
lazy_static! {
    #[doc = "Tells the codegen to remove dead code before export."]pub static ref DEAD_CODE_ELIMINATION:bool = {
        std::env::vars().into_iter().find_map(|(key,value)|if key == stringify!(DEAD_CODE_ELIMINATION){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(DEAD_CODE_ELIMINATION),value),
        }).unwrap_or(true)
    };
}
fn link_static_initializers(a: Option<&Method>, b: Option<&Method>) -> Option<Method> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (Some(a), Some(b)) => {
            let mut merged: Method = a.clone();
            let mut blocks = merged.blocks_mut();
            let trees = blocks[0].trees_mut();
            trees.pop();
            trees.extend(b.blocks()[0].trees().iter().cloned());
            drop(blocks);
            Some(merged)
        }
    }
}
