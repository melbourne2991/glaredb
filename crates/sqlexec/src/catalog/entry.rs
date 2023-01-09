//! Catalog entry types.
use crate::catalog::access::AccessMethod;
use crate::catalog::builtins::{BuiltinSchema, BuiltinTable, BuiltinView};
use datafusion::arrow::datatypes::{DataType, Field, Schema as ArrowSchema};
use std::ops::Deref;

/// Types of entries the catalog holds.
#[derive(Debug, Clone)]
pub enum EntryType {
    Schema,
    Table,
    View,
    Index,
    Sequence,
}

/// How the entry was created.
#[derive(Debug, Clone)]
pub enum EntryCreatedBy {
    /// The entry was created from a builtin object.
    System,
    /// The entry was created by the user. The SQL string will is the statement
    /// used to create the entry.
    User { sql: String },
}

impl From<String> for EntryCreatedBy {
    fn from(sql: String) -> Self {
        EntryCreatedBy::User { sql }
    }
}

/// Special entry type for dropping catalog entries.
#[derive(Debug)]
pub struct DropEntry {
    /// Entry type to drop.
    pub typ: EntryType,
    /// Schema the entry is in.
    pub schema: String,
    /// Name of the entry to drop. If dropping a schema, must be the same name
    /// as the schema.
    pub name: String,
}

/// An entry tagged with a catalog revision number.
#[derive(Debug)]
pub struct TaggedEntry<T: ?Sized> {
    pub revision: u64,
    pub inner: T,
}

impl<T: ?Sized> Deref for TaggedEntry<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct SchemaEntry {
    pub created_by: EntryCreatedBy,
    pub name: String,
}

impl From<&BuiltinSchema> for SchemaEntry {
    fn from(builtin: &BuiltinSchema) -> Self {
        SchemaEntry {
            created_by: EntryCreatedBy::System,
            name: builtin.name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableEntry {
    pub created_by: EntryCreatedBy,
    pub schema: String,
    pub name: String,
    pub access: AccessMethod,
    pub columns: Vec<ColumnDefinition>,
}

impl From<&BuiltinTable> for TableEntry {
    fn from(builtin: &BuiltinTable) -> Self {
        TableEntry {
            created_by: EntryCreatedBy::System,
            schema: builtin.schema.to_string(),
            name: builtin.name.to_string(),
            access: AccessMethod::System,
            columns: builtin.columns.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub datatype: DataType,
    pub nullable: bool,
}

impl ColumnDefinition {
    /// Convert an arrow schema into column definitions.
    pub fn from_arrow_schema(schema: &ArrowSchema) -> Vec<ColumnDefinition> {
        schema.fields().iter().map(|field| field.into()).collect()
    }

    /// Convert an iterator of arrow fields into column definitions.
    pub fn from_arrow_fields<'a, I>(fields: I) -> Vec<ColumnDefinition>
    where
        I: IntoIterator<Item = &'a Field>,
    {
        fields.into_iter().map(|field| field.into()).collect()
    }
}

impl From<&Field> for ColumnDefinition {
    fn from(field: &Field) -> Self {
        ColumnDefinition {
            name: field.name().clone(),
            datatype: field.data_type().clone(),
            nullable: field.is_nullable(),
        }
    }
}

impl From<&ColumnDefinition> for Field {
    fn from(col: &ColumnDefinition) -> Self {
        Field::new(&col.name, col.datatype.clone(), col.nullable)
    }
}

#[derive(Debug, Clone)]
pub struct ViewEntry {
    pub created_by: EntryCreatedBy,
    pub schema: String,
    pub name: String,
    pub sql: String,
}

impl From<&BuiltinView> for ViewEntry {
    fn from(builtin: &BuiltinView) -> Self {
        ViewEntry {
            created_by: EntryCreatedBy::System,
            schema: builtin.schema.to_string(),
            name: builtin.name.to_string(),
            sql: builtin.sql.to_string(),
        }
    }
}
