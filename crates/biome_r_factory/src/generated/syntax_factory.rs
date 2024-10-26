//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_r_syntax::{RSyntaxKind, RSyntaxKind::*, T, *};
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
#[derive(Debug)]
pub struct RSyntaxFactory;
impl SyntaxFactory for RSyntaxFactory {
    type Kind = RSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            R_BOGUS | R_BOGUS_EXPRESSION | R_BOGUS_PARAMETER | R_BOGUS_VALUE => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            R_BINARY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyRExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [+] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyRExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_BINARY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_BINARY_EXPRESSION, children)
            }
            R_DEFAULT_PARAMETER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == IDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [=] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyRExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_DEFAULT_PARAMETER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_DEFAULT_PARAMETER, children)
            }
            R_DOTS_PARAMETER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [...] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_DOTS_PARAMETER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_DOTS_PARAMETER, children)
            }
            R_DOUBLE_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == R_DOUBLE_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_DOUBLE_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_DOUBLE_VALUE, children)
            }
            R_FUNCTION_DEFINITION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![function] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if RParameters::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyRExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_FUNCTION_DEFINITION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_FUNCTION_DEFINITION, children)
            }
            R_IDENTIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == IDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_IDENTIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_IDENTIFIER, children)
            }
            R_IDENTIFIER_PARAMETER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == IDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_IDENTIFIER_PARAMETER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_IDENTIFIER_PARAMETER, children)
            }
            R_INTEGER_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == R_INTEGER_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_INTEGER_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_INTEGER_VALUE, children)
            }
            R_LOGICAL_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == R_LOGICAL_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_LOGICAL_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_LOGICAL_VALUE, children)
            }
            R_NULL_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == R_NULL_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_NULL_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_NULL_VALUE, children)
            }
            R_PARAMETERS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['('] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if RParameterList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![')'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_PARAMETERS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_PARAMETERS, children)
            }
            R_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![UNICODE_BOM] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if RExpressionList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(R_ROOT.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(R_ROOT, children)
            }
            R_STRING_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == R_STRING_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        R_STRING_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(R_STRING_VALUE, children)
            }
            R_EXPRESSION_LIST => {
                Self::make_node_list_syntax(kind, children, AnyRExpression::can_cast)
            }
            R_PARAMETER_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyRParameter::can_cast,
                T ! [,],
                false,
            ),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
