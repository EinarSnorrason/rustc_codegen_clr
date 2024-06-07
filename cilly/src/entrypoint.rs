use std::num::NonZeroU8;

use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    call,
    call_site::CallSite,
    call_virt,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_usize, ldc_u32, ldc_u64,
    method::{Attribute, Method, MethodType},
    size_of, DotnetTypeRef, FnSig, Type,
};

/// Creates a wrapper method around entypoint represented by `CallSite`
pub fn wrapper(entrypoint: &CallSite) -> Method {
    if entrypoint.signature().inputs()
        == [
            Type::ISize,
            Type::Ptr(Box::new(Type::Ptr(Box::new(Type::U8)))),
        ]
        && entrypoint.signature().output() == &Type::ISize
    {
        let sig = FnSig::new(
            &[Type::ManagedArray {
                element: Box::new(DotnetTypeRef::string_type().into()),
                dims: NonZeroU8::new(1).unwrap(),
            }],
            Type::Void,
        );
        let mem_checks = if crate::mem_checks() {
            CILRoot::Pop {
                tree: CILNode::Call {
                    site: Box::new(CallSite::mcheck()),
                    args: [conv_usize!(ldc_u64!(0))].into(),
                },
            }
        } else {
            CILRoot::Nop
        };
        let mut method = Method::new(
            AccessModifer::Public,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![(
                Some("argc".into()),
                Type::Ptr(Type::Ptr(Type::U8.into()).into()),
            )],
            vec![BasicBlock::new(
                vec![
                    mem_checks.into(),
                    CILRoot::Pop {
                        tree: call!(
                            Box::new(entrypoint.clone()),
                            [conv_usize!(ldc_u32!(0)), conv_usize!(ldc_u32!(0))]
                        ),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                2,
                None,
            )],
            vec![Some("args".into())],
        );
        //method.set_ops(ops);
        method.add_attribute(Attribute::EntryPoint);
        method
    } else if entrypoint.signature().inputs().is_empty()
        && entrypoint.signature().output() == &Type::Void
    {
        let sig = FnSig::new(&[], Type::Void);
        let mut method = Method::new(
            AccessModifer::Public,
            MethodType::Static,
            sig,
            "entrypoint",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::Call {
                        site: entrypoint.clone(),
                        args: [].into(),
                    }
                    .into(),
                    //CILRoot::debug(&format!("Preparing to execute the main program.")).into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
            vec![],
        );

        method.add_attribute(Attribute::EntryPoint);
        method
    } else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
