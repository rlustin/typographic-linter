use super::*;

pub struct PriceFilter {
    pub locale: String,
}

impl PriceFilter {
    fn currencies(&self) -> &'static str {
        "$¢£¤¥֏؋৲৳৻૱௹฿₠₡₢₣₤₥₦₧₨₩₪₫€₭₮₯₰₱₲₳₴₵₶₷₸₹₺₽﹩＄￠￡￥￦"
    }
}

impl LinterFilter for PriceFilter {
    fn locales(&self) -> Vec<&'static str> {
        vec!("de", "en", "es", "fr")
    }

    fn message(&self) -> &'static str {
        match &self.locale as &str {
            "de" | "es" | "fr" => "The currency sign should be written after the amount and a non-breaking space.",
            "en" => "The currency sign should be written before the amount without space.",
            "it" => "The currency sign should be written before the amount and a non-breaking space.",
            _ => unimplemented!(),
        }
    }

    fn regex_pattern(&self) -> String {
        match &self.locale as &str {
            // Matches one of the following:
            // - digits followed by a character (or none) other than a non-breaking space followed
            //   by any of currencies() return values (ex: `120€` or `120 $`);
            // - any of currencies() return values followed by any whitespace character (or not)
            //   followed by digits (ex: `€120` or `$ 120`).
            "de" | "es" | "fr" => format!("([\\d]+[^ ]?[{}]{{1}}|[{}]{{1}}[\\s]?[\\d]+)", self.currencies(), self.currencies()),

            // Matches one of the following:
            // - digits followed by any whitespace character (or not) followed by any of
            //   currencies() return values (ex: `120€` or `120 €`)
            // - any of currencies() return values followed by any whitespace character followed by
            //   digits (ex: `€ 120` or `$ 120`).
            "en" => format!("([\\d]+[\\s]?[{}]{{1}}|[{}]{{1}}[\\s][\\d]+)", self.currencies(), self.currencies()),

            // Matches one of the following:
            // - digits followed by any whitespace character (or not) followed by any of
            //   currencies() return values (ex: `120€` or `120 €`)
            // - any of currencies() return values followed by a character (or none) other than a
            //   non-breaking space followed by digits (ex: `€ 120` or `$120`).
            "it" => format!("([\\d]+[\\s]?[{}]{{1}}|[{}]{{1}}[^ ]?[\\d]+)", self.currencies(), self.currencies()),

            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExpectedWarning {
        locale: &'static str,
        text: &'static str,
        start: usize,
        end: usize,
    }

    fn expected_warnings() -> Vec<ExpectedWarning> {
        vec!(
            ExpectedWarning { locale: "de", text: "€120", start: 0, end: 6},
            ExpectedWarning { locale: "de", text: "120 €", start: 0, end: 7},
            ExpectedWarning { locale: "de", text: "120€", start: 0, end: 6},

            ExpectedWarning { locale: "en", text: "120 €", start: 0, end: 7},
            ExpectedWarning { locale: "en", text: "120 €", start: 0, end: 8},
            ExpectedWarning { locale: "en", text: "120€", start: 0, end: 6},
            ExpectedWarning { locale: "en", text: "€ 120", start: 0, end: 7},
            ExpectedWarning { locale: "en", text: "€ 120", start: 0, end: 8},

            ExpectedWarning { locale: "es", text: "€120", start: 0, end: 6},
            ExpectedWarning { locale: "es", text: "120 €", start: 0, end: 7},
            ExpectedWarning { locale: "es", text: "120€", start: 0, end: 6},

            ExpectedWarning { locale: "fr", text: "€120", start: 0, end: 6},
            ExpectedWarning { locale: "fr", text: "120 €", start: 0, end: 7},
            ExpectedWarning { locale: "fr", text: "120€", start: 0, end: 6},

            ExpectedWarning { locale: "it", text: "120 €", start: 0, end: 7},
            ExpectedWarning { locale: "it", text: "120 €", start: 0, end: 8},
            ExpectedWarning { locale: "it", text: "120€", start: 0, end: 6},
            ExpectedWarning { locale: "it", text: "€120", start: 0, end: 6},
            ExpectedWarning { locale: "it", text: "€ 120", start: 0, end: 7}
        )
    }

    #[test]
    fn test_filters_when_warnings() {
        for expected_warning in expected_warnings() {
            let filter = PriceFilter { locale: expected_warning.locale.to_string() };

            let result = filter.check(expected_warning.text);

            assert!(result.is_err());

            let warnings = result.err().unwrap();

            assert_eq!(1, warnings.len());
            assert_eq!(expected_warning.start, warnings[0].start);
            assert_eq!(expected_warning.end, warnings[0].end);
        }
    }

    #[test]
    fn test_filter_when_de_and_no_warnings() {
        let filter = PriceFilter { locale: "de".to_string() };

        let result = filter.check("120 €");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filter_when_en_and_no_warnings() {
        let filter = PriceFilter { locale: "en".to_string() };

        let result = filter.check("€120");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filter_when_es_and_no_warnings() {
        let filter = PriceFilter { locale: "es".to_string() };

        let result = filter.check("120 €");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filter_when_it_and_no_warnings() {
        let filter = PriceFilter { locale: "it".to_string() };

        let result = filter.check("€ 120");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filter_when_fr_and_no_warnings() {
        let filter = PriceFilter { locale: "fr".to_string() };

        let result = filter.check("120 €");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}

