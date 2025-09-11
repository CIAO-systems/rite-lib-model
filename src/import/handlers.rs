use crate::{BoxedError, record::Record};

use super::RecordHandler;

pub struct CollectingRecordHandler<'a> {
    records: &'a mut Vec<Record>,
}

impl<'a> CollectingRecordHandler<'a> {
    pub fn new(records: &'a mut Vec<Record>) -> Self {
        Self { records }
    }
}

impl<'a> RecordHandler for CollectingRecordHandler<'a> {
    fn handle_record(&mut self, record: &mut Record) -> Result<(), BoxedError> {
        self.records.push(Record::copy(record));
        Ok(())
    }
}

pub struct ClosureRecordHandler<F> {
    callback: F,
}

impl<F> ClosureRecordHandler<F>
where
    F: FnMut(&mut Record),
{
    pub fn new(callback: F) -> Self {
        if cfg!(test) {
            println!("This code is needed to cover inlined code!");
        }
        Self { callback }
    }
}

impl<F> RecordHandler for ClosureRecordHandler<F>
where
    F: FnMut(&mut Record),
{
    fn handle_record(&mut self, record: &mut Record) -> Result<(), BoxedError> {
        (self.callback)(record);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        field::add_field,
        import::{RecordHandler, handlers::ClosureRecordHandler},
        record::Record,
    };

    #[test]
    fn test_closure_handler() {
        let mut handler = ClosureRecordHandler::new(|r| {
            assert!(r.field_by_name("name").is_some());
            let field = r.field_by_name("name").unwrap();
            assert_eq!(field.value(), "value".into());
        });

        let mut r = Record::new();
        add_field(r.fields_as_mut(), "name", "value".into());
        let result = handler.handle_record(&mut r);
        assert!(result.is_ok());
    }
}
