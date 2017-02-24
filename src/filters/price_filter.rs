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
        vec!("fr")
    }

    fn message(&self) -> &'static str {
        match &self.locale as &str {
            "fr" => "The currency sign should be written after the amount and a non-breaking space.",
            _ => "",
        }
    }

    fn regex_pattern(&self) -> String {
        match &self.locale as &str {
            // Matches one of the following:
            // - digits followed by a character (or none) other than a non-breaking space followed
            //   by any of currencies() return values (ex: `120€` or `120 $`);
            // - any of currencies() return values followed by any whitespace character (or not)
            //   followed by digits (ex: `€120` or `$ 120`).
            "fr" => format!("([\\d]+[^ ]?[{}]{{1}}|[{}]{{1}}[\\s]?[\\d]+)", self.currencies(), self.currencies()),
            _ => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExpectedWarning {
        text: &'static str,
        start: usize,
        end: usize,
    }

    fn fr_expected_warnings() -> Vec<ExpectedWarning> {
        vec!(
            ExpectedWarning { text: "€120", start: 0, end: 6},
            ExpectedWarning { text: "120 €", start: 0, end: 7},
            ExpectedWarning { text: "120€", start: 0, end: 6}
        )
    }

    #[test]
    fn test_filters_when_fr_warnings() {
        for expected_warning in fr_expected_warnings() {
            let filter = PriceFilter { locale: "fr".to_string() };

            let result = filter.check(expected_warning.text);

            assert!(result.is_err());

            let warnings = result.err().unwrap();

            assert_eq!(1, warnings.len());
            assert_eq!("The currency sign should be written after the amount and a non-breaking space.", warnings[0].message);
            assert_eq!(expected_warning.start, warnings[0].start);
            assert_eq!(expected_warning.end, warnings[0].end);
        }
    }

    #[test]
    fn test_filter_when_fr_and_no_warnings() {
        let filter = PriceFilter { locale: "fr".to_string() };

        let result = filter.check("120 €");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}

