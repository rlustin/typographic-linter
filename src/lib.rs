extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod errors;
mod filters;

use errors::LinterWarning;
use filters::*;

pub struct Linter {
    locale: String,
}

impl Linter {
    pub fn new(locale: String) -> Result<Linter, LinterWarning> {
        let linter = Linter { locale: locale };

        Ok(linter)
    }

    pub fn check(&self, text: &str) -> Result<(), Vec<LinterWarning>> {
        let mut warnings = Vec::<LinterWarning>::new();

        for filter in &self.active_filters(self.locale.as_str()) {
            let result = filter.check(text);

            if result.is_err() {
                warnings.append(&mut result.err().unwrap());
            }
        }

        if warnings.is_empty() {
            Ok(())
        } else {
            Err(warnings)
        }
    }

    fn active_filters(&self, locale: &str) -> Vec<Box<LinterFilter>> {
        self.filters(locale)
            .into_iter()
            .filter(|filter| filter.locales().is_empty() || filter.locales().contains(&locale))
            .collect()
    }

    fn filters(&self, locale: &str) -> Vec<Box<LinterFilter>> {
        vec![
            Box::new(CurlyApostropheFilter {}),
            Box::new(EllipsisSymbolFilter {}),
            Box::new(NoSpaceBeforeCommaFilter {}),
            Box::new(PriceFilter { locale: locale.to_string() }),
            Box::new(QuotesFilter { locale: locale.to_string() }),
            Box::new(SpaceBeforeDoublePonctuationFilter {}),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_with_two_warnings() {
        let linter = Linter::new("en".to_string()).unwrap();

        let result = linter.check("It's me...");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(2, warnings.len());

        println!("WARNING 0: {:?}", warnings[0]);

        assert_eq!("Please use curly apostrophes.", warnings[0].message);
        assert_eq!(2, warnings[0].start);
        assert_eq!(3, warnings[0].end);

        assert_eq!("Please use the ellipsis symbol (`…`) instead of three dots (`...`).", warnings[1].message);
        assert_eq!(7, warnings[1].start);
        assert_eq!(10, warnings[1].end);
    }

    #[test]
    fn test_linter_with_no_warning() {
        let linter = Linter::new("en".to_string()).unwrap();

        let result = linter.check("It’s me…");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_linter_with_a_language_specific_warning() {
        let linter = Linter::new("fr".to_string()).unwrap();

        let result = linter.check("C’est moi!");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!(
            "Please use a non-breaking space before “double” ponctuation marks: `;`, `:`, `!`, `?`.",
            warnings[0].message
        );
        assert_eq!(10, warnings[0].start);
        assert_eq!(12, warnings[0].end);
    }

    #[test]
    fn test_linter_with_a_language_specific_filter_and_no_warning() {
        let linter = Linter::new("en".to_string()).unwrap();

        let result = linter.check("It’s me!");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}
