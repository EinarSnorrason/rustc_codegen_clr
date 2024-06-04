#![feature(iterator_try_collect)]
use std::collections::HashMap;

use cilly::{
    asm::Assembly, call_site::CallSite, cil_node::CILNode, cil_root::{CILRoot, SFI}, field_desc, method::Method, static_field_desc::StaticFieldDescriptor, FnSig, IString, Type
};
#[derive(Debug)]
enum Exception {
    MethodNotFound(CallSite),
    LocalOutOfRange { loc: usize, lcount: usize },
    ArgOutOfRange { arg: usize, lcount: usize },
    AllocOffsetOutOfRange,
}
type AllocID = u32;
struct InterpreterState<'asm> {
    asm: &'asm Assembly,
    call_stack: Vec<(&'asm CallSite, usize, usize,cilly::cil_root::SFI)>,
    locals: Vec<Box<[Value]>>,
    mem: HashMap<AllocID, Box<[u8]>>,
    last_alloc: AllocID,
    fields:HashMap<StaticFieldDescriptor,Value>,
    methods:HashMap<AllocID,CallSite>,
    inv_methods:HashMap<CallSite,AllocID>,
    last_alloc_method:AllocID,
}
#[derive(Clone, Debug,PartialEq)]
enum Value {
    Undef,
    StringArray(Box<[IString]>),
    String(IString),
    USize(usize),
    ISize(isize),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Ptr(AllocID, u32),
    Bool(bool),
    F32(f32),
    F64(f64),
    ValueType(AllocID, u32),
}

impl Value {

    fn as_usize(&self) -> Option<usize> {
        if let Self::USize(v) = self {
            Some(*v)
        } else if let Self::ISize(v) = self {
            Some(*v as usize)
        } else if let Self::I32(v) = self {
            Some(*v as isize as usize)
        } else if let Self::U32(v) = self {
            Some(*v as usize)
        }else if let Self::Ptr(alloc, offset) = self {
            Some((((*alloc as u64) << (32)) + *offset as u64) as usize)
        } else {
            None
        }
    }
    fn as_ptr(&self) -> Option<(AllocID, u32)> {
        if let Self::Ptr(id, offset) = self {
            Some((*id, *offset))
        } else {
            None
        }
    }

    fn as_string(&self) -> Option<&IString> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn as_u64(&self) -> Option<&u64> {
        if let Self::U64(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_i8(&self) -> Option<i8> {
        if let Self::I8(v) = self {
            Some(*v)
        } else if let Self::U8(v) = self {
            Some(*v as i8)
        } else if let Self::U32(v) = self {
            assert!(*v < 256);
            Some(*v as u8 as i8)
        }else {
            None
        }
    }
}
fn eval_node<'asm>(
    node: &'asm CILNode,
    state: &mut InterpreterState<'asm>,
    args: &mut Box<[Value]>,
) -> Result<Value, Exception> {
    assert_eq!(state.locals.len(), state.call_stack.len());
    match node {
        CILNode::LdcU64(val) => Ok(Value::U64(*val)),
        CILNode::LdcI64(val) => Ok(Value::I64(*val)),
        CILNode::LdcU32(val) => Ok(Value::U32(*val)),
        CILNode::LdcI32(val) => Ok(Value::I32(*val)),
        CILNode::LdcF64(val) => Ok(Value::F64(*val)),
        CILNode::LdcF32(val) => Ok(Value::F32(*val)),
        CILNode::LdFalse => Ok(Value::Bool(false)),
        CILNode::LdTrue => Ok(Value::Bool(true)),
        CILNode::SizeOf(tpe) => match tpe.as_ref() {
            Type::USize | Type::ISize => Ok(Value::I32(8)),
            _ => todo!("Can't yet calc sizeof {tpe:?}"),
        },
        CILNode::TransmutePtr { val, new_ptr: _ }=>match eval_node(val, state,args)?{
            Value::Ptr(alloc_id,offset)=>Ok(Value::Ptr(alloc_id,offset)),
            _=>todo!("Can't transmute {val:?} to pointer.")
        }
        CILNode::Call { args:call_args, site } | CILNode::CallVirt { args:call_args, site } | CILNode::NewObj { args:call_args, site } => {
            let mut call_args: Box<[_]> = call_args.iter().map(|arg| eval_node(arg, state,args)).try_collect()?;
            assert_eq!(state.locals.len(), state.call_stack.len());
            match state.run(site, &mut call_args) {
                Ok(val) => Ok(val),
                Err(Exception::MethodNotFound(_)) => state.try_call_extern(site, &mut call_args),
                Err(err) => Err(err),
            }
        }
        CILNode::Eq(a, b) => {
            let a = eval_node(a, state,args)?;
            let b = eval_node(b, state,args)?;
            match (&a,&b){
                (Value::I8(b),Value::I8(val))=>Ok(Value::Bool(*b == *val)),
                (Value::U8(b),Value::U8(val))=>Ok(Value::Bool(*b == *val)),
                (Value::I16(b),Value::I16(val))=>Ok(Value::Bool(*b == *val)),
                (Value::U16(b),Value::U16(val))=>Ok(Value::Bool(*b == *val)),
                (Value::I32(b),Value::I32(val))=>Ok(Value::Bool(*b == *val)),
                (Value::U32(b),Value::U32(val))=>Ok(Value::Bool(*b == *val)),
                (Value::Bool(a),Value::Bool(b))=>Ok(Value::Bool(*a == *b)),
                
                (Value::Bool(b),Value::I32(val))=>Ok(Value::Bool(*b as u8 as i32 == *val)),
                (Value::Bool(b),Value::U32(val))=>Ok(Value::Bool(*b as u8 as u32 == *val)),
                (Value::U16(b),Value::U32(val))=>Ok(Value::Bool(*b as u32 == *val)),
              
                _=>todo!("Can't yet cmp {a:?} and {b:?}")
            } 
        }
        CILNode::Lt(a, b) => {
            let a = eval_node(a, state,args)?;
            let b = eval_node(b, state,args)?;
            let res = match (&a, &b) {
                (Value::I32(a), Value::I32(b)) => Ok(Value::Bool(*a < *b)),
                (Value::F32(a), Value::F32(b)) => Ok(Value::Bool(*a < *b)),
                _ => todo!("Can't yet lt {a:?} and {b:?}"),
            };
            res
        }
        CILNode::Add(a, b) => {
            let a = eval_node(a, state,args)?;
            let b = eval_node(b, state,args)?;
            match (&a, &b) {
                (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a.wrapping_add(*b))),
                (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a.wrapping_add(*b))),
                (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a.wrapping_add(*b))),
                (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a.wrapping_add(*b))),
                (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a.wrapping_add(*b))),
                (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a.wrapping_add(*b))),
                (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a.wrapping_add(*b))),
                (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a.wrapping_add(*b))),
                (Value::I32(a), Value::USize(b)) => Ok(Value::USize((*a as usize).wrapping_add(*b))),
                (Value::Ptr(alloc, offset), Value::USize(b)) => Ok(Value::Ptr(
                    *alloc,
                    offset
                        .checked_add(*b as u32)
                        .ok_or(Exception::AllocOffsetOutOfRange)?,
                )),
                (Value::Ptr(alloc, offset), Value::U32(b)) => Ok(Value::Ptr(
                    *alloc,
                    offset
                        .checked_add(*b)
                        .ok_or(Exception::AllocOffsetOutOfRange)?,
                )),
                // Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't yet add {a:?} and {b:?}"),
            }
        }
        CILNode::LDElelemRef { arr, idx } => {
            let a = eval_node(arr, state,args)?;
            let b = eval_node(idx, state,args)?;
            match (&a, &b) {
                (Value::StringArray(arr), Value::I32(idx)) => {
                    Ok(Value::String(arr[*idx as usize].clone()))
                }
                // Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't yet index into {a:?} wtih {b:?}"),
            }
        }
        CILNode::Mul(a, b) => {
            let a = eval_node(a, state,args)?;
            let b = eval_node(b, state,args)?;
            match (&a, &b) {
                (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a.wrapping_mul(*b))),
                (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a.wrapping_mul(*b))),
                // Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't yet mul {a:?} and {b:?}"),
            }
        }
        
        CILNode::ConvI8(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I8(val) => Ok(Value::I8(val as i8)),
                Value::U8(val) => Ok(Value::I8(val as i8)),
                Value::I16(val) => Ok(Value::I8(val as i8)),
                Value::U16(val) => Ok(Value::I8(val as i8)),
                Value::I32(val) => Ok(Value::I8(val as i8)),
                Value::U32(val) => Ok(Value::I8(val as i8)),
                _ => todo!("Can't convert a value of type {value:?} to i8"),
            }
        }
        CILNode::MRefToRawPtr(ptr)=>eval_node(ptr, state, args),
        CILNode::ConvU8(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I8(val) => Ok(Value::U8(val as u8)),
                Value::U8(val) => Ok(Value::U8(val as u8)),
                Value::I16(val) => Ok(Value::U8(val as u8)),
                Value::U16(val) => Ok(Value::U8(val as u8)),
                Value::I32(val) => Ok(Value::U8(val as u8)),
                Value::U32(val) => Ok(Value::U8(val as u8)),
                Value::I64(val) => Ok(Value::U8(val as u8)),
                Value::U64(val) => Ok(Value::U8(val as u8)),
                _ => todo!("Can't convert a value of type {value:?} to u8"),
            }
        }
        CILNode::ConvI16(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I8(val) => Ok(Value::I16(val as i16)),
                Value::U8(val) => Ok(Value::I16(val as i16)),
                Value::I16(val) => Ok(Value::I16(val as i16)),
                Value::U16(val) => Ok(Value::I16(val as i16)),
                Value::I32(val) => Ok(Value::I16(val as i16)),
                Value::U32(val) => Ok(Value::I16(val as i16)),
                _ => todo!("Can't convert a value of type {value:?} to i16"),
            }
        }
        CILNode::Neg(value)=>{
            let value = eval_node(value, state,args)?;
            match value{
                Value::I64(val)=>Ok(Value::I64(-val)),
                _=>todo!("Can't neg {value:?}"),
            }
        }
        CILNode::ConvU16(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I8(val) => Ok(Value::U16(val as u16)),
                Value::U8(val) => Ok(Value::U16(val as u16)),
                Value::I16(val) => Ok(Value::U16(val as u16)),
                Value::U16(val) => Ok(Value::U16(val as u16)),
                Value::I32(val) => Ok(Value::U16(val as u16)),
                Value::U32(val) => Ok(Value::U16(val as u16)),
                _ => todo!("Can't convert a value of type {value:?} to u16"),
            }
        }
        CILNode::ConvI32(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I8(val) => Ok(Value::I32(val as i32)),
                Value::U8(val) => Ok(Value::I32(val as i32)),
                Value::I16(val) => Ok(Value::I32(val as i32)),
                Value::U16(val) => Ok(Value::I32(val as i32)),
                Value::I32(val) => Ok(Value::I32(val as i32)),
                Value::U32(val) => Ok(Value::I32(val as i32)),
                Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't convert a value of type {value:?} to i32"),
            }
        }
        CILNode::ConvU32(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I8(val) => Ok(Value::U32(val as u32)),
                Value::U8(val) => Ok(Value::U32(val as u32)),
                Value::I16(val) => Ok(Value::U32(val as u32)),
                Value::U16(val) => Ok(Value::U32(val as u32)),
                Value::I32(val) => Ok(Value::U32(val as u32)),
                Value::U32(val) => Ok(Value::U32(val as u32)),
                _ => todo!("Can't convert a value of type {value:?} to u32"),
            }
        }
       
        CILNode::ConvF32(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I32(val) => Ok(Value::F32(val as f32)),
                _ => todo!("Can't convert a value of type {value:?} to f32"),
            }
        }
        CILNode::ConvU64(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::U64(val) => Ok(Value::U64(val)),
                _ => todo!("Can't convert a value of type {value:?} to u64"),
            }
        }
        CILNode::ConvI64(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I64(val) => Ok(Value::I64(val)),
                _ => todo!("Can't convert a value of type {value:?} to u64"),
            }
        }
        CILNode::ConvISize(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::U64(val) => Ok(Value::ISize(val as i64 as isize)),
                Value::I64(val) => Ok(Value::ISize(val as isize)),
                _ => todo!("Can't convert a value of type {value:?} to isize"),
            }
        }
        CILNode::ZeroExtendToUSize(value) => {
            let value = eval_node(value, state,args)?;
            match value {
                Value::I32(val) => Ok(Value::USize(val as u32 as usize)),
                Value::U32(val) => Ok(Value::USize(val as usize)),
                _ => todo!("Can't convert a value of type {value:?} to usize"),
            }
        }
        CILNode::LDLen { arr } => {
            let arr = eval_node(arr, state,args)?;
            match arr {
                Value::StringArray(arr) => Ok(Value::USize(arr.len())),
                _ => todo!("Can't get the length of a {arr:?}"),
            }
        }
        CILNode::LDFtn(site)=>Ok(Value::Ptr(state.get_fn_ptr_alloc(site),0)),
        CILNode::LDArg(arg)=>args.get(*arg as usize)
        .cloned()
        .ok_or(Exception::ArgOutOfRange {
            arg: *arg as usize,
            lcount: args.len(),
        }),
        CILNode::LDStaticField(field_desc)=>Ok(state.fields.get(&field_desc).cloned().unwrap_or(Value::Undef)),
        CILNode::LDLoc(loc) => state
            .locals
            .last()
            .unwrap()
            .get(*loc as usize)
            .cloned()
            .ok_or(Exception::LocalOutOfRange  {
                loc: *loc as usize,
                lcount: state.locals.len(),
            }),
        _ => todo!("Can't handle node:{node:?}"),
    }
}
impl<'asm> InterpreterState<'asm> {
    pub fn get_fn_ptr_alloc(&mut self,site:&CallSite)->AllocID{
        *self.inv_methods.entry(site.clone()).or_insert_with(||{
            let new_method = self.last_alloc_method;
            self.methods.insert(new_method,site.clone());
            new_method
        })
    }
    pub fn alloc(&mut self, size: usize) -> AllocID {
        let new_alloc = self.last_alloc;
        self.last_alloc += 1;
        self.mem
            .insert(new_alloc, (0..size).map(|_| 0_u8).collect());
        new_alloc
    }
    pub fn try_call_extern(
        &mut self,
        call: &'asm CallSite,
        args: &mut Box<[Value]>,
    ) -> Result<Value, Exception> {
        assert_eq!(self.locals.len(), self.call_stack.len());
        let res = match (call.class(), call.name(), call.signature()) {
            (Some(_),"IsNaN",_)=>match args[0]{
                Value::F32(f)=>Ok(Value::Bool(f.is_nan())),
                Value::F64(f)=>Ok(Value::Bool(f.is_nan())),
                _=>panic!("Can't check if {:?} is nan.",args[0]),
            }
            (Some(_), "AlignedAlloc" | "AllocHGlobal", _) => {

                Ok(Value::Ptr(self.alloc(args[0].as_usize().unwrap()), 0))
            }
            (Some(_), "GetCommandLineArgs", _) => Ok(Value::StringArray(
                std::env::args().map(|arg| arg.into()).collect(),
            )),
           
            (Some(_), "get_Length", _) => {let arg = &args[0]; match arg{
                Value::StringArray(arr)=>Ok(Value::I32(arr.len() as u32 as i32)),
               _=>todo!("Can't get length of {arg:?}")
            }}
            (Some(tpe), ".ctor", _) => match tpe.name_path(){
                "System.UInt128"=>{
                    let (lower,upper) = (*args[0].as_u64().unwrap() as u128,*args[1].as_u64().unwrap() as u128);
                    //Ok(Value::U128(lower<<64))
                   
                    Ok(Value::U128(lower<<64 | upper))
                }
                _=>todo!("Can't newobj {tpe:?}")
            }
            (Some(_), "StringToCoTaskMemUTF8", _) => {
                let string = args[0].as_string().unwrap().clone();
                let alloc = self.alloc(string.len() + 1);
                let alloc = Value::Ptr(alloc, 0);
                self.get_memory_at_mut(alloc.clone())[..string.len()]
                    .copy_from_slice(string.as_bytes());
                Ok(alloc)
            }

            (Some(class), name, sig) => todo!("Can't yet call extern {call:?}"),
            _ => Err(Exception::MethodNotFound(call.clone())),
        };
       
        res
    }
    pub fn run_cctor(&mut self) -> Result<Value, Exception> {
        match self.asm.cctor() {
            Some(_) => self.run(
                Box::<CallSite>::leak(Box::new(CallSite::builtin(
                    ".cctor".into(),
                    FnSig::new(&[], Type::Void),
                    true,
                ))),
                &mut vec![].into(),
            ),
            None => Ok(Value::Undef),
        }
    }
    pub fn run_entypoint(&mut self) -> Result<Value, Exception> {
        let entry = self.asm.methods().find(|method|method.is_entrypoint());
        match entry {
            Some(entry) => self.run(
                Box::<CallSite>::leak(Box::new(entry.call_site())),
                &mut vec![Value::StringArray(std::env::args().map(|arg|arg.into()).collect())].into(),
            ),
            None => Ok(Value::Undef),
        }
    }
    pub fn method(&self, site: &'asm CallSite) -> Result<&'asm Method, Exception> {
        self.asm
            .functions()
            .get(site)
            .ok_or(Exception::MethodNotFound(site.clone()))
    }
    pub fn run(
        &mut self,
        call: &'asm CallSite,
        args: &mut Box<[Value]>,
    ) -> Result<Value, Exception> {
        assert_eq!(self.locals.len(), self.call_stack.len());
        assert!(args.len() >= self.method(call)?.sig().inputs().len());
        self.locals
            .push(vec![Value::Undef; self.method(call)?.locals().len()].into());
        let sfi:SFI = Box::new((0..1,0..1,"".into()));
        self.call_stack.push((call, 0, 0,sfi));
        loop {
            let (method, block, tree,curr_sfi) = if let Some(method) = self.call_stack.last_mut() {
                method
            } else {
                break;
            };
            let method = self
                .asm
                .functions()
                .get(method)
                .ok_or::<Exception>(Exception::MethodNotFound(method.clone()))?;
            let block_ref = &method.blocks()[*block];
            let tree_ref = &block_ref.trees()[*tree];
            match tree_ref.root() {
                CILRoot::GoTo { target, sub_target } => {
                    assert_eq!(*sub_target, 0);
                    let (method, block, tree,_) = self.call_stack.last_mut().unwrap();
                    *block = *target as usize;
                    *tree = 0;
                    continue;
                }
                CILRoot::BTrue { target, sub_target,cond } => {
                    assert_eq!(*sub_target, 0);
                    let cond = eval_node(cond, self,args)?.as_bool().unwrap();
                    if cond{
                        let (method, block, tree,_) = self.call_stack.last_mut().unwrap();
                        *block = *target as usize;
                        *tree = 0;
                        continue;
                    }
                }
                CILRoot::BFalse { target, sub_target,cond } => {
                    assert_eq!(*sub_target, 0);
                    let cond = eval_node(cond, self,args)?.as_bool().unwrap();
                    if !cond{
                        let (method, block, tree,_) = self.call_stack.last_mut().unwrap();
                        *block = *target as usize;
                        *tree = 0;
                        continue;
                    }
                }
                CILRoot::Nop=>(),
                CILRoot::STLoc { local, tree } => {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let val = eval_node(tree, self,args)?;
                    self.locals.last_mut().unwrap()[*local as usize] = val;
                }
                CILRoot::STArg { arg, tree } => {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let val = eval_node(tree, self,args)?;
                    args[*arg as usize] = val;
                }
                CILRoot::SetField { addr, value, desc }=> {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let val = eval_node(value, self,args)?;
                    let addr = eval_node(addr, self,args)?;
                    todo!()
                }
                CILRoot::SetStaticField { descr, value } => {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let val = eval_node(value, self,args)?;
                    self.fields.insert(descr.clone(), val);
                }
                CILRoot::Pop { tree } => {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    eval_node(tree, self,args)?;
                }
                CILRoot::STIndISize(addr, val) => {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let addr = eval_node(addr, self,args)?;
                    let val = eval_node(val, self,args)?;
                    self.get_memory_at_mut(addr)[..8]
                        .copy_from_slice(&val.as_usize().unwrap().to_le_bytes()[..])
                }
                CILRoot::STIndI8(addr, val) => {
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let addr = eval_node(addr, self,args)?;
                    let val = eval_node(val, self,args)?;
                    self.get_memory_at_mut(addr)[0] = val.as_i8().unwrap() as u8;
                
                }
                CILRoot::VoidRet=>{
                    self.locals.pop();
                    self.call_stack.pop();
                    return Ok(Value::Undef);
                }
                CILRoot::Ret{tree}=>{
                    let tree = eval_node(tree, self,args)?;
                    self.locals.pop();
                    self.call_stack.pop();
                    return Ok(tree);
                }
                CILRoot::SourceFileInfo(sfi)=>{
                    println!("{}:{:?}:{:?}",sfi.2,sfi.0,sfi.1);
                    *curr_sfi = sfi.clone(); 
                }
                CILRoot::Call { args:call_args, site } | CILRoot::CallVirt { args:call_args, site } => {
                    let mut call_args: Box<[_]> = call_args.iter().map(|arg| eval_node(arg, self,args)).try_collect()?;
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    match self.run(site, &mut call_args) {
                        Ok(val) => Ok(val),
                        Err(Exception::MethodNotFound(_)) => self.try_call_extern(site, &mut call_args),
                        Err(err) => Err(err),
                    }?;
                }
                _ => todo!("unhandled root {root:?}", root = tree_ref.root()),
            }
            let (_, _, tree,_) = self.call_stack.last_mut().unwrap();
            *tree += 1;
        }
        todo!()
    }
    fn new(asm: &'asm Assembly) -> Self {
        Self {
            asm,
            call_stack: vec![],
            locals: vec![],
            mem: HashMap::new(),
            last_alloc: 1,
            fields:HashMap::new(),
            methods: HashMap::new(),
            inv_methods: HashMap::new(),
            last_alloc_method: 1<<31,
            
        }
    }

    fn get_memory_at_mut(&mut self, addr: Value) -> &mut [u8] {
        let (id, offset) = addr.as_ptr().unwrap();
        &mut self.mem.get_mut(&id).unwrap()[offset as usize..]
    }
}
fn load_asm(mut file: impl std::io::Read) -> Assembly {
    let mut asm_bytes = Vec::with_capacity(0x100);
    file.read_to_end(&mut asm_bytes)
        .expect("ERROR: Could not load the assembly file!");
    let assembly =
        postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!");
    assembly
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let asm = load_asm(std::fs::File::open(path).unwrap());
    let mut interpreter = InterpreterState::new(&asm);
    interpreter.run_cctor().unwrap();
    interpreter.run_entypoint().unwrap();
}
