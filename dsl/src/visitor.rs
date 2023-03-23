//! A set of traits and functions for visiting all nodes in a library.
//!
//! To use the visitor, define a struct and implement the Visitor trait
//! for the struct.
//!
//! Visitor trait functions call functions that implement walking through
//! the library. Selectively call this functions to selectively descend
//! into the library.
//!
//! # Example
//!
//! ```
//! use ironplc_dsl::common::FunctionDeclaration;
//! use ironplc_dsl::visitor::{ Visitor, visit_function_declaration };
//!
//! struct Dummy {}
//! impl Dummy {
//!   fn do_work() {}
//! }
//!
//! impl Visitor<String> for Dummy {
//!     type Value = ();
//!
//!     fn visit_function_declaration(&mut self, node: &FunctionDeclaration) -> Result<Self::Value, String> {
//!         // Do something custom before visiting the FunctionDeclaration node
//!         Dummy::do_work();
//!
//!         // Continue the recursion
//!         visit_function_declaration(self, node)
//!     }
//! }
//! ```

use crate::common::*;
use crate::common_sfc::Network;
use crate::textual::*;

/// Defines a way to recurse into an object in the AST or DSL.
pub trait Acceptor {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E>;
}

/// Recurses into a vec of objects.
impl<X> Acceptor for Vec<X>
where
    X: Acceptor,
{
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self.iter().map(|x| x.accept(visitor)).find(|r| r.is_err()) {
            Some(err) => {
                // At least one of the items returned an error, so
                // return the first error.
                err
            }
            None => {
                // There were no errors, so return the default value
                Ok(V::Value::default())
            }
        }
    }
}

/// Recurses into an optional object. Does nothing if the option is none.
impl<X> Acceptor for Option<X>
where
    X: Acceptor,
{
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self.as_ref() {
            Some(x) => x.accept(visitor),
            None => Ok(V::Value::default()),
        }
    }
}

/// Defines a visitor for the object tree. The default visitor recursively
/// walks to visit items in the tree.
pub trait Visitor<E> {
    /// Value produced by this visitor when the result is not an error.
    ///
    /// The returned value is usually not meaningful because no guarantee
    /// is provided when returning from vectors of objects.
    type Value: Default;

    fn walk(&mut self, node: &Library) -> Result<Self::Value, E> {
        Acceptor::accept(&node.elements, self)
    }

    fn visit_signed_integer(&mut self, node: &SignedInteger) -> Result<Self::Value, E> {
        Ok(Self::Value::default())
    }

    fn visit_enumerated_value(&mut self, node: &EnumeratedValue) -> Result<Self::Value, E> {
        Ok(Self::Value::default())
    }

    fn visit_configuration_declaration(
        &mut self,
        node: &ConfigurationDeclaration,
    ) -> Result<Self::Value, E> {
        visit_configuration_declaration(self, node)
    }

    fn visit_resource_declaration(&mut self, node: &ResourceDeclaration) -> Result<Self::Value, E> {
        visit_resource_declaration(self, node)
    }

    fn visit_variable_declaration(&mut self, node: &VarDecl) -> Result<Self::Value, E> {
        visit_variable_declaration(self, node)
    }

    fn visit_enum_declaration(&mut self, node: &EnumerationDeclaration) -> Result<Self::Value, E> {
        visit_enum_declaration(self, node)
    }

    fn visit_subrange_declaration(&mut self, node: &SubrangeDeclaration) -> Result<Self::Value, E> {
        visit_subrange_declaration(self, node)
    }

    fn visit_simple_declaration(&mut self, node: &SimpleDeclaration) -> Result<Self::Value, E> {
        visit_simple_declaration(self, node)
    }

    fn visit_array_declaration(&mut self, node: &ArrayDeclaration) -> Result<Self::Value, E> {
        visit_array_declaration(self, node)
    }

    fn visit_structure_declaration(
        &mut self,
        node: &StructureDeclaration,
    ) -> Result<Self::Value, E> {
        visit_structure_declaration(self, node)
    }

    fn visit_structure_initialization_declaration(
        &mut self,
        node: &StructureInitializationDeclaration,
    ) -> Result<Self::Value, E> {
        visit_structure_initialization_declaration(self, node)
    }

    fn visit_structure_element_init(
        &mut self,
        node: &StructureElementInit,
    ) -> Result<Self::Value, E> {
        visit_structure_element_init(self, node)
    }

    fn visit_string_declaration(&mut self, node: &StringDeclaration) -> Result<Self::Value, E> {
        visit_string_declaration(self, node)
    }

    fn visit_late_bound_declaration(
        &mut self,
        node: &LateBoundDeclaration,
    ) -> Result<Self::Value, E> {
        visit_late_bound_declaration(self, node)
    }

    fn visit_function_block_declaration(
        &mut self,
        node: &FunctionBlockDeclaration,
    ) -> Result<Self::Value, E> {
        visit_function_block_declaration(self, node)
    }

    fn visit_function_declaration(&mut self, node: &FunctionDeclaration) -> Result<Self::Value, E> {
        visit_function_declaration(self, node)
    }

    fn visit_program_declaration(&mut self, node: &ProgramDeclaration) -> Result<Self::Value, E> {
        visit_program_declaration(self, node)
    }

    fn visit_sfc(&mut self, node: &Sfc) -> Result<Self::Value, E> {
        Acceptor::accept(&node.networks, self)
    }

    fn visit_statements(&mut self, node: &Statements) -> Result<Self::Value, E> {
        Acceptor::accept(&node.body, self)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<Self::Value, E> {
        Acceptor::accept(&node.target, self)
    }

    fn visit_if(&mut self, node: &If) -> Result<Self::Value, E> {
        visit_if(self, node)
    }

    fn visit_case(&mut self, node: &Case) -> Result<Self::Value, E> {
        visit_case(self, node)
    }

    fn visit_case_statement_group(&mut self, node: &CaseStatementGroup) -> Result<Self::Value, E> {
        visit_case_statement_group(self, node)
    }

    fn visit_compare(&mut self, node: &CompareExpr) -> Result<Self::Value, E> {
        visit_compare(self, node)
    }

    fn visit_binary_op(&mut self, node: &BinaryExpr) -> Result<Self::Value, E> {
        // TODO this doesn't really go through binary operators - maybe this should be split
        // into smaller pieces.
        visit_binary_op(self, node)
    }

    fn visit_unary_op(&mut self, node: &UnaryExpr) -> Result<Self::Value, E> {
        visit_unary_op(self, node)
    }

    fn visit_address_assignment(&mut self, node: &AddressAssignment) -> Result<Self::Value, E> {
        todo!()
    }

    fn visit_symbolic_variable(&mut self, node: &SymbolicVariable) -> Result<Self::Value, E> {
        // leaf node - no children
        Ok(Self::Value::default())
    }

    fn visit_fb_call(&mut self, fb_call: &FbCall) -> Result<Self::Value, E> {
        // TODO
        Ok(Self::Value::default())
    }

    fn visit_simple_initializer(&mut self, init: &SimpleInitializer) -> Result<Self::Value, E> {
        // TODO
        Ok(Self::Value::default())
    }

    fn visit_string_initializer(&mut self, init: &StringInitializer) -> Result<Self::Value, E> {
        // TODO
        Ok(Self::Value::default())
    }

    fn visit_enumerated_type_initializer(
        &mut self,
        init: &EnumeratedInitialValueAssignment,
    ) -> Result<Self::Value, E> {
        // leaf node - no children
        Ok(Self::Value::default())
    }

    fn visit_enumerated_values_initializer(
        &mut self,
        init: &EnumeratedValuesInitializer,
    ) -> Result<Self::Value, E> {
        // leaf node - no children
        Ok(Self::Value::default())
    }

    fn visit_function_block_type_initializer(
        &mut self,
        init: &FunctionBlockInitialValueAssignment,
    ) -> Result<Self::Value, E> {
        // leaf node - no children
        Ok(Self::Value::default())
    }

    fn visit_enumerated_spec_init(
        &mut self,
        node: &EnumeratedSpecificationInit,
    ) -> Result<Self::Value, E> {
        Acceptor::accept(&node.spec, self)
        //TODO visit the default value
        //Acceptor::accept(spec_init.default, self)
    }

    fn visit_subrange_specification(
        &mut self,
        node: &SubrangeSpecification,
    ) -> Result<Self::Value, E> {
        visit_subrange_specification(self, node)
    }

    fn visit_subrange(&mut self, init: &Subrange) -> Result<Self::Value, E> {
        Ok(Self::Value::default())
    }

    fn visit_array_initializer(
        &mut self,
        init: &ArrayInitialValueAssignment,
    ) -> Result<Self::Value, E> {
        //TODO recurse into the children
        Ok(Self::Value::default())
    }
}

pub fn visit_configuration_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &ConfigurationDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.global_var, v)?;
    Acceptor::accept(&node.resource_decl, v)
}

pub fn visit_resource_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &ResourceDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.global_vars, v)
    // TODO there are more child elements here
}

pub fn visit_variable_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &VarDecl,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.initializer, v)
}

pub fn visit_enum_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &EnumerationDeclaration,
) -> Result<V::Value, E> {
    v.visit_enumerated_spec_init(&node.spec_init)
}

pub fn visit_subrange_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &SubrangeDeclaration,
) -> Result<V::Value, E> {
    v.visit_subrange_specification(&node.spec)
}

pub fn visit_simple_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &SimpleDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.spec_and_init, v)
}

pub fn visit_array_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &ArrayDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.spec, v)?;
    Acceptor::accept(&node.init, v)
}

pub fn visit_subrange_specification<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &SubrangeSpecification,
) -> Result<V::Value, E> {
    v.visit_subrange(&node.subrange)
}

pub fn visit_structure_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &StructureDeclaration,
) -> Result<V::Value, E> {
    // TODO
    Ok(V::Value::default())
}

pub fn visit_structure_initialization_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &StructureInitializationDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.elements_init, v)
}

pub fn visit_structure_element_init<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &StructureElementInit,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.init, v)
}

pub fn visit_string_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &StringDeclaration,
) -> Result<V::Value, E> {
    Ok(V::Value::default())
}

pub fn visit_late_bound_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &LateBoundDeclaration,
) -> Result<V::Value, E> {
    Ok(V::Value::default())
}

pub fn visit_function_block_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &FunctionBlockDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.variables, v)?;
    Acceptor::accept(&node.body, v)
}

pub fn visit_program_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &ProgramDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.variables, v)?;
    Acceptor::accept(&node.body, v)
}

pub fn visit_function_declaration<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &FunctionDeclaration,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.variables, v)?;
    Acceptor::accept(&node.body, v)
}

pub fn visit_if<V: Visitor<E> + ?Sized, E>(v: &mut V, node: &If) -> Result<V::Value, E> {
    Acceptor::accept(&node.expr, v)?;
    Acceptor::accept(&node.body, v)?;
    Acceptor::accept(&node.else_body, v)
}

pub fn visit_case<V: Visitor<E> + ?Sized, E>(v: &mut V, node: &Case) -> Result<V::Value, E> {
    Acceptor::accept(&node.selector, v)?;
    Acceptor::accept(&node.statement_groups, v)?;
    Acceptor::accept(&node.else_body, v)
}

pub fn visit_case_statement_group<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &CaseStatementGroup,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.selectors, v)?;
    Acceptor::accept(&node.statements, v)
}

pub fn visit_compare<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &CompareExpr,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.left, v)?;
    Acceptor::accept(&node.right, v)
}

pub fn visit_binary_op<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &BinaryExpr,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.left, v)?;
    Acceptor::accept(&node.right, v)
}

pub fn visit_unary_op<V: Visitor<E> + ?Sized, E>(
    v: &mut V,
    node: &UnaryExpr,
) -> Result<V::Value, E> {
    Acceptor::accept(&node.term, v)
}

impl Acceptor for LibraryElement {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            LibraryElement::ConfigurationDeclaration(config) => {
                visitor.visit_configuration_declaration(config)
            }
            LibraryElement::DataTypeDeclaration(data_type_decl) => {
                Acceptor::accept(data_type_decl, visitor)
            }
            LibraryElement::FunctionBlockDeclaration(func_block_decl) => {
                visitor.visit_function_block_declaration(func_block_decl)
            }
            LibraryElement::FunctionDeclaration(func_decl) => {
                visitor.visit_function_declaration(func_decl)
            }
            LibraryElement::ProgramDeclaration(prog_decl) => {
                visitor.visit_program_declaration(prog_decl)
            }
        }
    }
}

impl Acceptor for ResourceDeclaration {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        visitor.visit_resource_declaration(self)
    }
}

impl Acceptor for VarDecl {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        visitor.visit_variable_declaration(self)
    }
}

impl Acceptor for EnumerationDeclaration {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        visitor.visit_enum_declaration(self)
    }
}

impl Acceptor for DataTypeDeclarationKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            DataTypeDeclarationKind::Enumeration(e) => visitor.visit_enum_declaration(e),
            DataTypeDeclarationKind::Subrange(sr) => visitor.visit_subrange_declaration(sr),
            DataTypeDeclarationKind::Simple(simple) => visitor.visit_simple_declaration(simple),
            DataTypeDeclarationKind::Array(a) => visitor.visit_array_declaration(a),
            DataTypeDeclarationKind::Structure(s) => visitor.visit_structure_declaration(s),
            DataTypeDeclarationKind::StructureInitialization(si) => {
                visitor.visit_structure_initialization_declaration(si)
            }
            DataTypeDeclarationKind::String(s) => visitor.visit_string_declaration(s),
            DataTypeDeclarationKind::LateBound(bound) => {
                visitor.visit_late_bound_declaration(bound)
            }
        }
    }
}

impl Acceptor for EnumeratedSpecificationKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        // TODO I don't know if we need to visit these items
        Ok(V::Value::default())
    }
}

impl Acceptor for ArrayInitialElementKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            ArrayInitialElementKind::Constant(_) => todo!(),
            ArrayInitialElementKind::EnumValue(_) => todo!(),
            ArrayInitialElementKind::Repeated(_, _) => todo!(),
        }
    }
}

impl Acceptor for ArraySpecificationKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            ArraySpecificationKind::Type(_) => todo!(),
            ArraySpecificationKind::Subranges(_, _) => todo!(),
        }
    }
}

impl Acceptor for InitialValueAssignmentKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            InitialValueAssignmentKind::None => Ok(V::Value::default()),
            InitialValueAssignmentKind::Simple(si) => visitor.visit_simple_initializer(si),
            InitialValueAssignmentKind::String(str) => visitor.visit_string_initializer(str),
            InitialValueAssignmentKind::EnumeratedValues(ev) => {
                visitor.visit_enumerated_values_initializer(ev)
            }
            InitialValueAssignmentKind::EnumeratedType(et) => {
                visitor.visit_enumerated_type_initializer(et)
            }
            InitialValueAssignmentKind::FunctionBlock(fbi) => {
                visitor.visit_function_block_type_initializer(fbi)
            }
            InitialValueAssignmentKind::Subrange(sr) => visitor.visit_subrange_specification(sr),
            InitialValueAssignmentKind::Structure(si) => {
                visitor.visit_structure_initialization_declaration(si)
            }
            InitialValueAssignmentKind::Array(array_init) => {
                visitor.visit_array_initializer(array_init)
            }
            InitialValueAssignmentKind::LateResolvedType(_) => Ok(V::Value::default()),
        }
        // TODO don't yet know how to visit these
    }
}

impl Acceptor for StructureElementInit {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        visitor.visit_structure_element_init(self)
    }
}

impl Acceptor for StructInitialValueAssignmentKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            StructInitialValueAssignmentKind::Constant(_) => todo!(),
            StructInitialValueAssignmentKind::EnumeratedValue(_) => todo!(),
            StructInitialValueAssignmentKind::Array(_) => todo!(),
            StructInitialValueAssignmentKind::Structure(_) => todo!(),
        }
    }
}

impl Acceptor for ExprKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            ExprKind::Compare(compare) => visitor.visit_compare(compare.as_ref()),
            ExprKind::BinaryOp(binary) => visitor.visit_binary_op(binary.as_ref()),
            ExprKind::UnaryOp(unary) => visitor.visit_unary_op(unary.as_ref()),
            ExprKind::Const(_) => {
                todo!()
            }
            ExprKind::Expression(_) => {
                todo!()
            }
            ExprKind::Variable(variable) => Acceptor::accept(variable, visitor),
            ExprKind::Function {
                name,
                param_assignment,
            } => {
                todo!()
            }
        }
    }
}

impl Acceptor for FunctionBlockBody {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            FunctionBlockBody::Sfc(network) => visitor.visit_sfc(network),
            FunctionBlockBody::Statements(stmts) => visitor.visit_statements(stmts),
            // TODO it isn't clear if visiting this is necessary
            FunctionBlockBody::Empty() => Ok(V::Value::default()),
        }
    }
}

impl Acceptor for StmtKind {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            StmtKind::Assignment(node) => visitor.visit_assignment(node),
            StmtKind::FbCall(node) => visitor.visit_fb_call(node),
            StmtKind::If(node) => visitor.visit_if(node),
            StmtKind::Case(node) => visitor.visit_case(node),
            StmtKind::For(_) => todo!(),
            StmtKind::While(_) => todo!(),
            StmtKind::Repeat(_) => todo!(),
            StmtKind::Exit => todo!(),
        }
    }
}

impl Acceptor for CaseStatementGroup {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        visitor.visit_case_statement_group(self)
    }
}

impl Acceptor for CaseSelection {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            CaseSelection::Subrange(sr) => visitor.visit_subrange(sr),
            CaseSelection::SignedInteger(si) => visitor.visit_signed_integer(si),
            CaseSelection::EnumeratedValue(ev) => visitor.visit_enumerated_value(ev),
        }
    }
}

impl Acceptor for Network {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        // TODO
        Ok(V::Value::default())
    }
}

impl Acceptor for Variable {
    fn accept<V: Visitor<E> + ?Sized, E>(&self, visitor: &mut V) -> Result<V::Value, E> {
        match self {
            Variable::AddressAssignment(var) => visitor.visit_address_assignment(var),
            Variable::SymbolicVariable(var) => visitor.visit_symbolic_variable(var),
            Variable::MultiElementVariable(_) => {
                todo!()
            }
        }
    }
}

mod test {
    use super::*;
    use crate::common::*;
    use crate::core::{Id, SourceLoc};
    use crate::textual::*;
    use std::collections::LinkedList;
    use std::fmt::Error;

    struct Descender {
        names: LinkedList<String>,
    }
    impl Descender {
        fn new() -> Descender {
            Descender {
                names: LinkedList::new(),
            }
        }
    }

    impl Visitor<Error> for Descender {
        type Value = ();

        fn visit_address_assignment(&mut self, variable: &AddressAssignment) -> Result<(), Error> {
            let mut dst = &mut self.names;
            dst.push_back(variable.to_string());
            Ok(())
        }

        fn visit_symbolic_variable(&mut self, var: &SymbolicVariable) -> Result<(), Error> {
            let mut dst = &mut self.names;
            dst.push_back(var.name.to_string());
            Ok(())
        }

        fn visit_fb_call(&mut self, fb_call: &FbCall) -> Result<(), Error> {
            let mut dst = &mut self.names;
            dst.push_back(fb_call.var_name.to_string());
            Ok(())
        }
    }

    #[test]
    fn walk_when_has_symbolic_variable_then_visits_variable() {
        let library = Library {
            elements: vec![LibraryElement::ProgramDeclaration(ProgramDeclaration {
                type_name: Id::from("plc_prg"),
                variables: vec![VarDecl::simple("Reset", "BOOL").with_type(VariableType::Input)],
                body: FunctionBlockBody::stmts(vec![StmtKind::fb_assign(
                    "AverageVal",
                    vec!["Cnt1", "Cnt2"],
                    "_TMP_AverageVal17_OUT",
                )]),
            })],
        };

        let mut descender = Descender::new();

        descender.walk(&library);

        assert_eq!(1, descender.names.len())
    }
}
