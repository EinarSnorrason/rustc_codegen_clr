use super::AssemblyExporter;
use crate::cil_tree::cil_root::CILRoot;

use crate::r#type::TypeDef;
use crate::{
    cil_tree::{cil_node::CILNode, CILTree},
    method::Method,
    r#type::Type,
    IString,
};
use std::collections::HashMap;
use std::process::Command;
use std::{borrow::Cow, collections::HashSet, io::Write};
pub struct CExporter {
    types: Vec<u8>,
    type_defs: Vec<u8>,
    method_defs: Vec<u8>,
    static_defs: Vec<u8>,
    encoded_asm: Vec<u8>,
    headers: Vec<u8>,
    defined: HashSet<IString>,
    delayed_typedefs: HashMap<IString, TypeDef>,
}
impl std::io::Write for CExporter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.encoded_asm.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.encoded_asm.flush()
    }
}

impl CExporter {
    fn as_source(&self, is_dll: bool) -> Vec<u8> {
        let mut res = self.headers.clone();
        res.extend(&self.types);
        res.extend(&self.type_defs);
        res.extend(&self.method_defs);
        res.extend(&self.static_defs);
        res.extend(&self.encoded_asm);
        if !is_dll {
            writeln!(res, "int main(){{_cctor();entrypoint();}}").unwrap();
        }
        res
    }
    fn add_method_inner(&mut self, method: &crate::method::Method, class: Option<String>) {
        //eprintln!("C source:\n{}",String::from_utf8_lossy(&self.as_source()));
        let sig = method.sig();

        let name = method.name().replace('.', "_");
        // Puts is already defined in C.
        if name == "puts"
            || name == "malloc"
            || name == "printf"
            || name == "free"
            || name == "realloc"
        {
            return;
        }
        let output = c_tpe(sig.output());
        let mut inputs: String = "(".into();
        let mut input_iter = sig
            .inputs()
            .iter()
            .enumerate()
            .filter(|(_, tpe)| **tpe != Type::Void);
        if let Some((idx, input)) = input_iter.next() {
            inputs.push_str(&format!("{input} A{idx}", input = c_tpe(input)));
        }
        for (idx, input) in input_iter {
            inputs.push_str(&format!(",{input} A{idx} ", input = c_tpe(input)));
        }
        inputs.push(')');
        let mut code = String::new();
        for (id, (_, local)) in method.locals().iter().enumerate() {
            if *local == Type::Void {
                continue;
            }
            code.push_str(&format!("\t{local} L{id};\n", local = c_tpe(local)));
        }
        for bb in method.blocks() {
            code.push_str(&format!("\tBB_{}:\n", bb.id()));
            for tree in bb.trees() {
                code.push_str(&format!("{}\n", tree_string(tree, method)));
                //code.push_str(&format!("/*{tree:?}*/\n"));
            }
        }
        if let Some(class) = class {
            let class = class.replace('.', "_");
            writeln!(self.method_defs, "{output} {class}{name} {inputs};").unwrap();
            write!(
                self.encoded_asm,
                "{output} {class}{name} {inputs}{{\n{code}}}\n"
            )
            .unwrap();
        } else {
            writeln!(self.method_defs, "{output} {name} {inputs};").unwrap();
            write!(self.encoded_asm, "{output} {name} {inputs}{{\n{code}}}\n").unwrap();
        }
    }
}
impl AssemblyExporter for CExporter {
    fn init(_asm_info: &super::AssemblyInfo) -> Self {
        let mut encoded_asm = Vec::with_capacity(0x1_00);
        let types = Vec::with_capacity(0x1_00);
        let type_defs = Vec::with_capacity(0x1_00);
        let method_defs = Vec::with_capacity(0x1_00);
        let static_defs = Vec::with_capacity(0x1_00);
        let mut headers = Vec::with_capacity(0x1_00);
        write!(headers, "/*  This file was autogenerated by `rustc_codegen_clr` by FractalFir\n It contains C code made from Rust.*/\n").expect("Write error!");

        write!(
            headers,
            "#include  <stdint.h>\n#include <stdbool.h>\n#include <stddef.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <mm_malloc.h>\n"
        )
        .expect("Write error!");
        headers.write_all(include_bytes!("c_header.h")).unwrap();
        writeln!(headers).expect("Write error!");
        writeln!(
            encoded_asm,
            "#pragma GCC diagnostic ignored \"-Wmaybe-uninitialized\""
        )
        .unwrap();
        writeln!(
            encoded_asm,
            "#pragma GCC diagnostic ignored \"-Wunused-label\""
        )
        .unwrap();
        writeln!(
            encoded_asm,
            "#pragma GCC diagnostic ignored \"-Wunused-but-set-variable\""
        )
        .unwrap();
        writeln!(
            encoded_asm,
            "#pragma GCC diagnostic ignored \"-Wunused-variable\""
        )
        .unwrap();
        writeln!(
            encoded_asm,
            "#pragma GCC diagnostic ignored \"-Wpointer-sign\""
        )
        .unwrap();
        Self {
            types,
            type_defs,
            encoded_asm,
            method_defs,
            static_defs,
            headers,
            defined: HashSet::new(),
            delayed_typedefs: HashMap::new(),
        }
    }
    fn add_type(&mut self, tpe: &crate::r#type::TypeDef) {
        let name = tpe.name();
        if self.defined.contains(name) {
            return;
        }
        for tpe_name in tpe
            .fields()
            .iter()
            .filter_map(|field| field.1.as_dotnet())
            .filter_map(|tpe| {
                if tpe.asm().is_none() {
                    Some(tpe.name_path().to_owned())
                } else {
                    None
                }
            })
        {
            if !self.defined.contains::<Box<_>>(&tpe_name.clone().into()) {
                //eprintln!("type {tpe_name:?} has unresolved dependencies");
                self.delayed_typedefs.insert(name.into(), tpe.clone());
                return;
            }
        }
        let mut fields = String::new();
        if let Some(offsets) = tpe.explicit_offsets() {
            for ((field_name, field_type), offset) in tpe.fields().iter().zip(offsets) {
                if *field_type == Type::Void {
                    continue;
                }

                fields.push_str(&format!(
                    "\tstruct {{char pad[{offset}];{field_type} f;}} {field_name};\n\n",
                    field_type = c_tpe(field_type)
                ));
            }
        } else {
            for (field_name, field_type) in tpe.fields() {
                if *field_type == Type::Void {
                    continue;
                }
                fields.push_str(&format!(
                    "\tstruct {{{field_type} f;}} {field_name};\n",
                    field_type = c_tpe(field_type)
                ));
            }
        }
        for method in tpe.methods() {
            self.add_method_inner(method, Some(name.to_owned()));
        }
        if tpe.explicit_offsets().is_some() {
            writeln!(self.types, "typedef union {name} {name};").unwrap();
            write!(self.type_defs, "union {name}{{\n{fields}}};\n").unwrap()
        } else {
            writeln!(self.types, "typedef struct {name} {name};").unwrap();
            write!(self.type_defs, "struct {name}{{\n{fields}}};\n").unwrap()
        }
        self.defined.insert(name.into());
        let delayed_typedefs = self.delayed_typedefs.clone();
        self.delayed_typedefs = HashMap::new();
        for (_, tpe) in delayed_typedefs {
            self.add_type(&tpe);
        }
    }

    fn add_method(&mut self, method: &crate::method::Method) {
        self.add_method_inner(method, None)
    }

    fn add_extern_method(&mut self, _lib_path: &str, name: &str, sig: &crate::function_sig::FnSig) {
        if name == "puts" || name == "malloc" || name == "printf" || name == "free" {
            return;
        }
        let output = c_tpe(sig.output());
        let mut inputs: String = "(".into();
        let mut input_iter = sig
            .inputs()
            .iter()
            .enumerate()
            .filter(|(_, tpe)| **tpe != Type::Void);
        if let Some((idx, input)) = input_iter.next() {
            inputs.push_str(&format!("{input} A{idx}", input = c_tpe(input)));
        }
        for (idx, input) in input_iter {
            inputs.push_str(&format!(",{input} A{idx} ", input = c_tpe(input)));
        }
        inputs.push(')');
        writeln!(self.method_defs, "extern {output} {name} {inputs};").unwrap();
    }

    fn finalize(
        self,
        final_path: &std::path::Path,
        is_dll: bool,
    ) -> Result<(), super::AssemblyExportError> {
        let cc = "gcc";
        let src_path = final_path.with_extension("c");
        std::fs::File::create(&src_path)
            .unwrap()
            .write_all(&self.as_source(is_dll))
            .unwrap();
        let sanitize = if *crate::config::C_SANITIZE {
            "-fsanitize=undefined"
        } else {
            "-O"
        };
        let out = Command::new(cc)
            .args([
                "-g",
                sanitize,
                "-o",
                &final_path.to_string_lossy().to_owned(),
                &src_path.to_string_lossy().to_owned(),
            ])
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("error") {
            panic!("C compiler error:{stderr:?}!");
        }
        Ok(())
    }

    fn add_extern_ref(&mut self, _asm_name: &str, _info: &crate::assembly::AssemblyExternRef) {
        // Not needed in C
    }

    fn add_global(&mut self, tpe: &crate::r#type::Type, name: &str) {
        writeln!(self.static_defs, "static {tpe} {name};", tpe = c_tpe(tpe)).unwrap();
    }
}
fn node_string(tree: &CILNode) -> String {
    match tree {
        CILNode::LDLoc(loc) => format!("L{loc}"),
        CILNode::LDArg(arg) => format!("A{arg}"),
        CILNode::LDLocA(arg) => format!("((uintptr_t)(void*)&L{arg})"),
        CILNode::LDArgA(loc) => format!("((uintptr_t)(void*)&A{loc})"),
        CILNode::BlackBox(inner) => node_string(inner),
        CILNode::LDStaticField(static_field) => static_field.name().into(),
        CILNode::ConvF32(inner) => format!("((float){inner})", inner = node_string(inner)),
        CILNode::ConvF64(inner) | CILNode::ConvF64Un(inner) => {
            format!("((double){inner})", inner = node_string(inner))
        }
        CILNode::SizeOf(tpe) => format!("sizeof({tpe})", tpe = c_tpe(tpe)),
        CILNode::LDIndI8 { ptr } => format!("(*((int8_t*){ptr}))", ptr = node_string(ptr)),
        CILNode::LDIndI16 { ptr } => format!("(*((int16_t*){ptr}))", ptr = node_string(ptr)),
        CILNode::LDIndI32 { ptr } => format!("(*((int32_t*){ptr}))", ptr = node_string(ptr)),
        CILNode::LDIndI64 { ptr } => format!("(*((int64_t*){ptr}))", ptr = node_string(ptr)),
        CILNode::LDIndISize { ptr } => format!("(*((ptrdiff_t*){ptr}))", ptr = node_string(ptr)),
        CILNode::LdObj { ptr, obj } => format!(
            "(*({owner}*)({ptr}))",
            ptr = node_string(ptr),
            owner = c_tpe(obj)
        ),
        CILNode::LDIndF32 { ptr } => format!("(*((float*){ptr}))", ptr = node_string(ptr)),
        CILNode::LDIndF64 { ptr } => format!("(*((double*){ptr}))", ptr = node_string(ptr)),
        CILNode::LDFieldAdress { addr, field } => format!(
            "(&(({owner}*){ptr})->{name}.f)",
            ptr = node_string(addr),
            owner = c_tpe(&field.owner().clone().into()),
            name = field.name()
        ),
        CILNode::LDField { addr, field } => format!(
            "(({owner}*){ptr})->{name}.f",
            ptr = node_string(addr),
            owner = c_tpe(&field.owner().clone().into()),
            name = field.name()
        ),
        CILNode::Add(a, b) => format!("({a}) + ({b})", a = node_string(a), b = node_string(b)),
        CILNode::And(a, b) => format!("({a}) & ({b})", a = node_string(a), b = node_string(b)),
        CILNode::Sub(a, b) => format!("({a}) - ({b})", a = node_string(a), b = node_string(b)),
        CILNode::Mul(a, b) => format!("({a}) * ({b})", a = node_string(a), b = node_string(b)),
        CILNode::Div(a, b) => format!("({a}) / ({b})", a = node_string(a), b = node_string(b)),
        CILNode::Rem(a, b) | CILNode::RemUn(a, b) => {
            format!("{a} % {b}", a = node_string(a), b = node_string(b))
        }
        CILNode::Or(a, b) => format!("({a}) | ({b})", a = node_string(a), b = node_string(b)),
        CILNode::XOr(a, b) => format!("({a}) ^ ({b})", a = node_string(a), b = node_string(b)),
        CILNode::Shr(a, b) => format!("{a} >> {b}", a = node_string(a), b = node_string(b)),
        CILNode::Shl(a, b) | CILNode::ShrUn(a, b) => {
            format!("{a} << {b}", a = node_string(a), b = node_string(b))
        }
        CILNode::RawOpsParrentless { .. } => todo!(),
        CILNode::Call { args, site } => {
            let name = site.name();
            let mut input_iter = args
                .iter()
                .zip(site.signature().inputs())
                .filter_map(|(code, tpe)| if *tpe != Type::Void { Some(code) } else { None });
            let mut inputs: String = "(".into();
            if let Some(input) = input_iter.next() {
                inputs.push_str(&node_string(input).to_string());
            }
            for input in input_iter {
                inputs.push_str(&format!(",{input} ", input = node_string(input)));
            }
            inputs.push(')');
            let tpe_name = site
                .class()
                .map(|tpe| tpe.name_path())
                .unwrap_or("")
                .replace('.', "_");
            format!("{tpe_name}{name}{inputs}")
        }
        CILNode::CallVirt { .. } => panic!("Virtual calls not supported in C."),
        CILNode::LdcI64(value) => format!("{value}l"),
        CILNode::LdcU64(value) => format!("{value}ul"),
        CILNode::LdcI32(value) => format!("{value}"),
        CILNode::LdcU32(value) => format!("{value}u"),
        CILNode::LdcF64(value) => format!("{value}"),
        CILNode::LdcF32(value) => format!("{value}"),
        CILNode::LoadGlobalAllocPtr { .. } => todo!(),
        CILNode::ConvU8(inner) => format!("((uint8_t){inner})", inner = node_string(inner)),
        CILNode::ConvU16(inner) => format!("((uint16_t){inner})", inner = node_string(inner)),
        CILNode::ConvU32(inner) => format!("((uint32_t){inner})", inner = node_string(inner)),
        CILNode::ConvU64(inner) => format!("((uint64_t){inner})", inner = node_string(inner)),
        CILNode::ConvUSize(inner) => format!("((uintptr_t){inner})", inner = node_string(inner)),
        CILNode::ConvI8(inner) => format!("((int8_t){inner})", inner = node_string(inner)),
        CILNode::ConvI16(inner) => format!("((int16_t){inner})", inner = node_string(inner)),
        CILNode::ConvI32(inner) => format!("((int32_t){inner})", inner = node_string(inner)),
        CILNode::ConvI64(inner) => format!("((int64_t){inner})", inner = node_string(inner)),
        CILNode::ConvISize(inner) => format!("((ptrdiff_t){inner})", inner = node_string(inner)),
        CILNode::Neg(a) => format!("-({a})", a = node_string(a)),
        CILNode::Not(a) => format!("!({a})", a = node_string(a)),
        CILNode::Eq(a, b) => format!("(({a}) == ({b}))", a = node_string(a), b = node_string(b)),
        CILNode::Lt(a, b) | CILNode::LtUn(a, b) => {
            format!("{a} < {b}", a = node_string(a), b = node_string(b))
        }
        CILNode::Gt(a, b) | CILNode::GtUn(a, b) => {
            format!("{a} > {b}", a = node_string(a), b = node_string(b))
        }
        CILNode::TemporaryLocal(_) => todo!(),
        CILNode::SubTrees(sub, main) => {
            assert!(sub.is_empty(), "A sub-tree still remains!");
            println!(
                "WARNING: Sub-trees impropely resolved: an empty sub-tree list still remains!"
            );
            node_string(main)
        }
        CILNode::LoadAddresOfTMPLocal => todo!(),
        CILNode::LoadTMPLocal => todo!(),
        CILNode::LDFtn(_) => todo!(),
        CILNode::LDTypeToken(_) => todo!(),
        CILNode::NewObj { site, args } => {
            let mut input_iter = args
                .iter()
                .zip(site.signature().inputs())
                .filter_map(|(code, tpe)| if *tpe != Type::Void { Some(code) } else { None });
            let mut inputs: String = "(".into();
            if let Some(input) = input_iter.next() {
                inputs.push_str(&node_string(input).to_string());
            }
            for input in input_iter {
                inputs.push_str(&format!(",{input} ", input = node_string(input)));
            }
            inputs.push(')');
            let tpe_name = site.class().unwrap().name_path().replace('.', "_");
            format!("ctor_{tpe_name}{inputs}")
        }
        CILNode::LdStr(string) => format!("{string:?}"),
        CILNode::CallI {
            sig: _,
            fn_ptr: _,
            args: _,
        } => todo!(),
    }
}
fn tree_string(tree: &CILTree, method: &Method) -> String {
    match tree.root() {
        CILRoot::STLoc { local, tree } => {
            let local_ty = &method.locals()[*local as usize].1;
            if let Some(_) = local_ty.as_dotnet() {
                format!("\tL{local} = {tree};\n", tree = node_string(tree))
            } else {
                format!(
                    "\tL{local} = (({local_ty}){tree});\n",
                    tree = node_string(tree),
                    local_ty = c_tpe(local_ty)
                )
            }
        }
        CILRoot::BTrue {
            target,
            sub_target,
            ops,
        } => {
            if *sub_target != 0 {
                format!("\tif({ops})goto BB_{sub_target};\n", ops = node_string(ops))
            } else {
                format!("\tif({ops})goto BB_{target};\n", ops = node_string(ops))
            }
        }
        CILRoot::GoTo { target, sub_target } => {
            if *sub_target != 0 {
                format!("goto BB_{sub_target};")
            } else {
                format!("goto BB_{target};")
            }
        }
        CILRoot::Call { site, args } => {
            let name = site.name();
            let mut input_iter = args
                .iter()
                .zip(site.signature().inputs())
                .filter(|(_, tpe)| **tpe != Type::Void);
            let mut inputs: String = "(".into();
            if let Some((input, arg)) = input_iter.next() {
                if let Some(_) = arg.as_dotnet() {
                    inputs.push_str(&node_string(input).to_string());
                } else {
                    inputs.push_str(&format!(
                        "({arg})({ops})",
                        ops = node_string(input),
                        arg = c_tpe(arg)
                    ));
                }
                //                inputs.push_str(&format!("{input}", input = node_string(input)));
            }
            for (input, arg) in input_iter {
                if let Some(_) = arg.as_dotnet() {
                    // Can't cast to a struct in C.
                    inputs.push_str(&format!(",{ops}", ops = node_string(input)));
                } else {
                    inputs.push_str(&format!(
                        ",({arg})({ops})",
                        ops = node_string(input),
                        arg = c_tpe(arg)
                    ));
                }
            }
            inputs.push(')');
            let tpe_name = site
                .class()
                .map(|tpe| tpe.name_path())
                .unwrap_or("")
                .replace('.', "_");
            format!("{tpe_name}{name}{inputs};")
        }
        CILRoot::SetField { addr, value, desc } => format!(
            "(({owner}*){ptr})->{name}.f = {value};",
            ptr = node_string(addr),
            owner = c_tpe(&desc.owner().clone().into()),
            name = desc.name(),
            value = node_string(value)
        ),
        CILRoot::SetTMPLocal { value } => {
            panic!("Temporary locals must be resolved before the export stage! value:{value:?}")
        }
        CILRoot::CpBlk { src, dst, len } => format!(
            "memcpy({src},{dst},{len});",
            src = node_string(src),
            dst = node_string(dst),
            len = node_string(len)
        ),
        CILRoot::STIndI8(addr_calc, value_calc) => format!(
            "*((int8_t*)({addr_calc})) = (int8_t){value_calc};",
            addr_calc = node_string(addr_calc),
            value_calc = node_string(value_calc)
        ),
        CILRoot::STIndI16(_, _) => todo!(),
        CILRoot::STIndI32(_, _) => todo!(),
        CILRoot::STIndI64(_, _) => todo!(),
        CILRoot::STIndISize(addr_calc, value_calc) => format!(
            "*((uintptr_t*)({addr_calc})) = (uintptr_t){value_calc};",
            addr_calc = node_string(addr_calc),
            value_calc = node_string(value_calc)
        ),
        CILRoot::STIndF64(_, _) => todo!(),
        CILRoot::STIndF32(_, _) => todo!(),
        CILRoot::STObj {
            tpe,
            addr_calc,
            value_calc,
        } => {
            let local_ty = tpe;
            if let Some(_) = local_ty.as_dotnet() {
                format!(
                    "*(({local_ty}*)({addr_calc})) = {value_calc};",
                    addr_calc = node_string(addr_calc),
                    value_calc = node_string(value_calc),
                    local_ty = c_tpe(local_ty)
                )
            } else {
                format!(
                    "*(({local_ty}*)({addr_calc})) = (({local_ty}){value_calc});",
                    addr_calc = node_string(addr_calc),
                    value_calc = node_string(value_calc),
                    local_ty = c_tpe(local_ty)
                )
            }
        }
        CILRoot::STArg { arg, tree } => {
            format!("A{arg} = {tree};", tree = node_string(tree))
        }
        CILRoot::Break => "".into(),
        CILRoot::Nop => "".into(),
        CILRoot::InitBlk { dst, val, count } => {
            todo!("Can't memset yet. dst:{dst:?} val:{val:?} count:{count:?}")
        }
        CILRoot::CallVirt { .. } => panic!("Virtual calls not supported in C."),
        CILRoot::Ret { tree } => {
            if let Some(_) = method.sig().output().as_dotnet() {
                format!("\treturn {ops};", ops = node_string(tree))
            } else {
                format!(
                    "\treturn ({ret}){ops};",
                    ops = node_string(tree),
                    ret = c_tpe(method.sig().output())
                )
            }
        }
        CILRoot::Pop { tree } => {
            format!("\t{ops};", ops = node_string(tree))
        }
        CILRoot::VoidRet => "return;".into(),
        CILRoot::Throw(_) => "abort();".to_string(),
        CILRoot::ReThrow => todo!(),
        CILRoot::CallI { sig, fn_ptr, args } => todo!(
            "Can't yet call function pointers in C. fn_ptr:{fn_ptr:?} sig:{sig:?} args:{args:?}"
        ),
        CILRoot::JumpingPad { ops } => {
            println!("WARNING: There should be no jumping pads in C, jet a jumping pad remains! ops:{ops:?}");
            "/*Invalid jump pad was here*/abort();\n".into()
        }
        CILRoot::SetStaticField { descr, value } => {
            let local_ty = descr.tpe();
            if let Some(_) = local_ty.as_dotnet() {
                format!(
                    "{name} = {value_calc};",
                    name = descr.name(),
                    value_calc = node_string(value)
                )
            } else {
                format!(
                    "{name} = (({local_ty}){value_calc});",
                    name = descr.name(),
                    value_calc = node_string(value),
                    local_ty = c_tpe(local_ty)
                )
            }
        }
    }
}
fn c_tpe(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Bool => "bool".into(),
        Type::USize => "uintptr_t".into(),
        Type::ISize => "ptrdiff_t".into(),
        Type::Void => "void".into(),
        Type::I128 => "__int128".into(),
        Type::U128 => "unsigned __int128".into(),
        Type::I64 => "int64_t".into(),
        Type::U64 => "uint64_t".into(),
        Type::I32 => "int32_t".into(),
        Type::U32 => "uint32_t".into(),
        Type::F64 => "float".into(),
        Type::F32 => "double".into(),
        Type::I16 => "int16_t".into(),
        Type::U16 => "uint16_t".into(),
        Type::I8 => "int8_t".into(),
        Type::U8 => "uint8_t".into(),
        Type::Ptr(inner) => format!("{inner}*", inner = c_tpe(inner)).into(),
        Type::DotnetType(tref) => {
            if let Some(asm) = tref.asm() {
                match (asm, tref.name_path()) {
                    ("System.Runtime", "System.UInt128") => return c_tpe(&Type::U128),
                    ("System.Runtime", "System.Int128") => return c_tpe(&Type::I128),
                    _ => println!("Type {tref:?} is not supported in C"),
                }
            }
            // Ugly hack to deal with `c_void`
            if tref.name_path().contains("c_void")
                && tref.name_path().contains("ffi")
                && tref.name_path().contains("core")
            {
                return c_tpe(&Type::Void);
            }
            tref.name_path().replace('.', "_").to_owned().into()
        }
        Type::DelegatePtr(sig) => {
            let mut input_iter = sig.inputs().iter().filter(|tpe| **tpe != Type::Void);
            let mut inputs: String = "(".into();
            if let Some(input) = input_iter.next() {
                inputs.push_str(&format!("{input}", input = c_tpe(input)));
            }
            for input in input_iter {
                inputs.push_str(&format!(",{input} ", input = c_tpe(input)));
            }
            inputs.push(')');
            format!("({output} (*){inputs})", output = c_tpe(sig.output())).into()
        }
        _ => todo!("Unsuported type {tpe:?}"),
    }
}