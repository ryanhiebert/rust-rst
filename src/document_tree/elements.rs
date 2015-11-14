use url::Url;

use super::extra_attributes::{self,ExtraAttributes};
use super::element_categories::*;


//-----------------\\
//Element hierarchy\\
//-----------------\\

pub trait Element {
	fn ids(&self)     -> &Vec<String>;
	fn names(&self)   -> &Vec<String>;
	fn source(&self)  -> &Option<Url>;
	fn classes(&self) -> &Vec<String>;
}

#[derive(Default,Debug)]
pub struct CommonAttributes {
	ids: Vec<String>,
	names: Vec<String>,
	//left out dupnames
	source: Option<Url>,
	classes: Vec<String>,
}

//----\\
//impl\\
//----\\

macro_rules! impl_element(($name:ident) => {
	impl Element for $name {
		fn ids(&self)     -> &Vec<String> { &self.common.ids }
		fn names(&self)   -> &Vec<String> { &self.common.names }
		fn source(&self)  -> &Option<Url> { &self.common.source }
		fn classes(&self) -> &Vec<String> { &self.common.classes }
	}
});

macro_rules! impl_children(($name:ident, $childtype:ident) => {
	impl HasChildren<$childtype> for $name {
		fn add_child<R: Into<$childtype>>(&mut self, child: R) {
			self.children.push(Box::new(child.into()));
		}
	}
});

macro_rules! impl_extra(($name:ident) => {
	impl ExtraAttributes<extra_attributes::$name> for $name {
		fn extra(&self) -> &extra_attributes::$name { &self.extra }
	}
});

macro_rules! impl_elem(
	($name:ident) => {
		#[derive(Default,Debug)]
		pub struct $name { common: CommonAttributes }
		impl_element!($name);
	};
	($name:ident; +) => {
		#[derive(Default,Debug)]
		pub struct $name { common: CommonAttributes, extra: extra_attributes::$name }
		impl_element!($name); impl_extra!($name);
	};
	($name:ident; *) => { //same as above with no default
		#[derive(Debug)]
		pub struct $name { common: CommonAttributes, extra: extra_attributes::$name }
		impl_element!($name); impl_extra!($name);
	};
	($name:ident, $childtype:ident) => {
		#[derive(Default,Debug)]
		pub struct $name { common: CommonAttributes, children: Vec<Box<$childtype>> }
		impl_element!($name); impl_children!($name, $childtype);
	};
	($name:ident, $childtype:ident; +) => {
		#[derive(Default,Debug)]
		pub struct $name { common: CommonAttributes, extra: extra_attributes::$name, children: Vec<Box<$childtype>> }
		impl_element!($name); impl_extra!($name); impl_children!($name, $childtype);
	};
);

macro_rules! impl_elems(( $( ($($args:tt)*) )* ) => {
	$( impl_elem!($($args)*); )*
});

impl_elems!(
	//structual elements
	(Section, SubSection)
	(Topic,   SubTopic)
	(Sidebar, SubSidebar)
	
	//structural subelements
	(Title,      TextOrInlineElement)
	(Subtitle,   TextOrInlineElement)
	(Decoration, DecorationElement)
	(Docinfo,    BibliographicElement)
	(Transition)
	
	//bibliographic elements
	(Author,       TextOrInlineElement)
	(Authors,      AuthorInfo)
	(Organization, TextOrInlineElement)
	(Address,      TextOrInlineElement; +)
	(Contact,      TextOrInlineElement)
	(Version,      TextOrInlineElement)
	(Revision,     TextOrInlineElement)
	(Status,       TextOrInlineElement)
	(Date,         TextOrInlineElement)
	(Copyright,    TextOrInlineElement)
	(Field,        SubField)
	
	//decoration elements
	(Header, BodyElement)
	(Footer, BodyElement)
	
	//simple body elements
	(Paragraph,              TextOrInlineElement)
	(LiteralBlock,           TextOrInlineElement; +)
	(DoctestBlock,           TextOrInlineElement; +)
	(MathBlock)
	(Rubric,                 TextOrInlineElement)
	(SubstitutionDefinition, TextOrInlineElement; +)
	(Comment,                TextOrInlineElement; +)
	(Pending)
	(Target; +)
	(Raw; +)
	(Image; *)
	
	//compound body elements
	(Compound,  BodyElement)
	(Container, BodyElement)
	
	(BulletList,     ListItem; +)
	(EnumeratedList, ListItem; +)
	(DefinitionList, DefinitionListItem)
	(FieldList,      Field)
	(OptionList,     OptionListItem)
	
	(LineBlock,     SubLineBlock)
	(BlockQuote,    SubBlockQuote)
	(Admonition,    SubTopic)
	(Attention,     BodyElement)
	(Hint,          BodyElement)
	(Note,          BodyElement)
	(Caution,       BodyElement)
	(Danger,        BodyElement)
	(Error,         BodyElement)
	(Important,     BodyElement)
	(Tip,           BodyElement)
	(Warning,       BodyElement)
	(Footnote,      SubFootnote; +)
	(Citation,      SubFootnote; +)
	(SystemMessage, BodyElement; +)
	(Figure,        SubFigure; +)
	(Table; +) //TODO
	
	//body sub elements
	(ListItem, BodyElement)
	
	(DefinitionListItem, SubDLItem)
	(Term,               TextOrInlineElement)
	(Classifier,         TextOrInlineElement)
	(Definition,         BodyElement)
	
	(FieldName, TextOrInlineElement)
	(FieldBody, BodyElement)
	
	(OptionListItem, SubOptionListItem)
	(OptionGroup,    Option_)
	(Description,    BodyElement)
	(Option_,        SubOption)
	(OptionString,   TextOrInlineElement)
	(OptionArgument, TextOrInlineElement; +)
	
	(Line,        TextOrInlineElement)
	(Attribution, TextOrInlineElement)
	(Label_,      TextOrInlineElement)
	
	(Caption, TextOrInlineElement)
	(Legend,  BodyElement)
	
	//inline elements
	(Emphasis,              TextOrInlineElement)
	(Literal,               TextOrInlineElement)
	(Reference,             TextOrInlineElement; +)
	(Strong,                TextOrInlineElement)
	(FootnoteReference,     TextOrInlineElement; +)
	(CitationReference,     TextOrInlineElement; +)
	(SubstitutionReference, TextOrInlineElement; +)
	(TitleReference,        TextOrInlineElement)
	(Abbreviation,          TextOrInlineElement)
	(Acronym,               TextOrInlineElement)
	(Superscript,           TextOrInlineElement)
	(Subscript,             TextOrInlineElement)
	(Inline,                TextOrInlineElement)
	(Problematic,           TextOrInlineElement; +)
	(Generated,             TextOrInlineElement)
	(Math)
	
	//also have non-inline versions. Inline image is no figure child, inline target has content
	(TargetInline, TextOrInlineElement; +)
	(RawInline; +)
	(ImageInline; *)
	
	//text element
	(TextElement)
);
