use mago_ast::ast::*;
use mago_ast::Node;
use mago_ast::Program;
use mago_interner::ThreadedInterner;
use mago_names::Names;
use mago_php_version::PHPVersion;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_span::HasSpan;
use mago_span::Position;
use mago_span::Span;

#[derive(Debug)]
pub struct Context<'a> {
    pub interner: &'a ThreadedInterner,
    pub version: PHPVersion,
    program: &'a Program,
    names: &'a Names,
    issues: IssueCollection,
    ancestors: Vec<Span>,
    pub hint_depth: usize,
}

impl<'a> Context<'a> {
    pub fn new(interner: &'a ThreadedInterner, version: PHPVersion, program: &'a Program, names: &'a Names) -> Self {
        Self { interner, version, program, names, issues: IssueCollection::default(), ancestors: vec![], hint_depth: 0 }
    }

    pub fn program(&self) -> Node<'a> {
        Node::Program(self.program)
    }

    pub fn report(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn lookup_name(&self, position: &Position) -> &'a str {
        self.interner.lookup(self.names.get(position))
    }

    pub fn lookup_hint(&self, hint: &Hint) -> String {
        match hint {
            Hint::Identifier(identifier) => self.lookup_name(&identifier.span().start).to_owned(),
            Hint::Parenthesized(parenthesized_hint) => {
                format!("({})", self.lookup_hint(&parenthesized_hint.hint))
            }
            Hint::Nullable(nullable_hint) => format!("?{}", self.lookup_hint(&nullable_hint.hint)),
            Hint::Union(union_hint) => {
                format!("{}|{}", self.lookup_hint(&union_hint.left), self.lookup_hint(&union_hint.right))
            }
            Hint::Intersection(intersection_hint) => {
                format!("{}&{}", self.lookup_hint(&intersection_hint.left), self.lookup_hint(&intersection_hint.right))
            }
            Hint::Null(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::True(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::False(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::Array(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::Callable(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::Static(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::Self_(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::Parent(keyword) => self.interner.lookup(&keyword.value).to_owned(),
            Hint::Void(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Never(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Float(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Bool(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Integer(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::String(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Object(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Mixed(identifier) => self.interner.lookup(&identifier.value).to_owned(),
            Hint::Iterable(identifier) => self.interner.lookup(&identifier.value).to_owned(),
        }
    }

    pub fn push_ancestor(&mut self, node: Span) {
        self.ancestors.push(node);
    }

    pub fn get_ancestors_len(&self) -> usize {
        self.ancestors.len()
    }

    pub fn get_ancestor(&self, index: usize) -> Span {
        self.ancestors[index]
    }

    pub fn pop_ancestor(&mut self) {
        self.ancestors.pop();
    }

    pub fn take_issue_collection(self) -> IssueCollection {
        self.issues
    }
}
