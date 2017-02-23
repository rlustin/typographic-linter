use super::*;

pub struct QuotesFilter {
    pub locale: String,
}

impl LinterFilter for QuotesFilter {
    fn locales(&self) -> Vec<&'static str> {
        vec!(
            "de",
            "en",
            "es",
            "fr",
            "it"
        )
    }

    fn message(&self) -> &'static str {
        match &self.locale as &str {
            "de" => "Please use german quotation marks without spaces.",
            "en" => "Please use english double quotation marks without spaces.",
            "es" => "Please use french quotation marks without spaces.",
            "fr" => "Please use french quotation marks with non-breaking spaces.",
            "it" => "Please use french quotation marks without spaces.",
            _ => "",
        }
    }

    fn regex_pattern(&self) -> &'static str {
        match &self.locale as &str {
            "de" => "(\".+\")|(«.+»)|(“.+”)|(„[\\s].+[\\s]“)",
            "en" => "(\".+\")|(«.+»)|(“[\\s].+[\\s]”)|(„.+“)",
            "es" => "(\".+\")|(«[\\s].+[\\s]»)|(“.+”)|(„.+“)",
            "fr" => "(\".+\")|(«[^ ].+[^ ]»)|(“.+”)|(„.+“)",
            "it" => "(\".+\")|(«\\s.+\\s»)|(“.+”)|(„.+“)",
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filters_when_de_and_straight_quotation_marks() {
        let filter = QuotesFilter { locale: "de".to_string() };

        let result = filter.check("\"Ich auch\", sagte der italienische");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use german quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(10, warnings[0].end);
    }

    #[test]
    fn test_filters_when_de_and_french_quotation_marks() {
        let filter = QuotesFilter { locale: "de".to_string() };

        let result = filter.check("«Ich auch», sagte der italienische");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use german quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(12, warnings[0].end);
    }

    #[test]
    fn test_filters_when_de_and_english_quotation_marks() {
        let filter = QuotesFilter { locale: "de".to_string() };

        let result = filter.check("“Ich auch”, sagte der italienische");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use german quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(14, warnings[0].end);
    }

    #[test]
    fn test_filters_when_de_and_german_quotation_marks_and_spaces() {
        let filter = QuotesFilter { locale: "de".to_string() };

        let result = filter.check("„ Ich auch “, sagte der italienische");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use german quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(16, warnings[0].end);
    }

    #[test]
    fn test_filter_when_de_and_no_warnings() {
        let filter = QuotesFilter { locale: "de".to_string() };

        let result = filter.check("„Ich auch“, sagte der italienische");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filters_when_es_and_straight_quotation_marks() {
        let filter = QuotesFilter { locale: "es".to_string() };

        let result = filter.check("\"Y yo también\", dijo el italiano");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(15, warnings[0].end);
    }

    #[test]
    fn test_filters_when_es_and_english_quotation_marks() {
        let filter = QuotesFilter { locale: "es".to_string() };

        let result = filter.check("“Y yo también”, dijo el italiano");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(19, warnings[0].end);
    }

    #[test]
    fn test_filters_when_es_and_french_quotation_marks_with_spaces() {
        let filter = QuotesFilter { locale: "es".to_string() };

        let result = filter.check("« Y yo también », dijo el italiano");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(19, warnings[0].end);
    }

    #[test]
    fn test_filter_when_es_and_no_warnings() {
        let filter = QuotesFilter { locale: "es".to_string() };

        let result = filter.check("«Y yo también», dijo el italiano");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filters_when_en_and_and_straight_quotation_marks() {
        let filter = QuotesFilter { locale: "en".to_string() };

        let result = filter.check("\"Mee too\", said the French.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use english double quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(9, warnings[0].end);
    }

    #[test]
    fn test_filters_when_en_and_and_french_quotation_marks() {
        let filter = QuotesFilter { locale: "en".to_string() };

        let result = filter.check("« Mee too », said the French.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use english double quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(15, warnings[0].end);
    }

    #[test]
    fn test_filters_when_en_and_and_english_quotation_marks_with_spaces() {
        let filter = QuotesFilter { locale: "en".to_string() };

        let result = filter.check("“ Mee too ”, said the French.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use english double quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(16, warnings[0].end);
    }

    #[test]
    fn test_filters_when_en_and_and_german_quotation_marks() {
        let filter = QuotesFilter { locale: "en".to_string() };

        let result = filter.check("„Mee too“, said the French.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use english double quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(13, warnings[0].end);
    }

    #[test]
    fn test_filter_when_en_and_no_warnings() {
        let filter = QuotesFilter { locale: "en".to_string() };

        let result = filter.check("“Mee too”, said the French.");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filters_when_fr_and_straight_quotation_marks() {
        let filter = QuotesFilter { locale: "fr".to_string() };

        let result = filter.check("\"Et moi aussi\", dit l’Anglais.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks with non-breaking spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(14, warnings[0].end);
    }

    #[test]
    fn test_filters_when_fr_and_english_quotation_marks() {
        let filter = QuotesFilter { locale: "fr".to_string() };

        let result = filter.check("“Et moi aussi”, dit l’Anglais.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks with non-breaking spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(18, warnings[0].end);
    }

    #[test]
    fn test_filters_when_fr_and_french_quotation_marks_without_non_breaking_spaces() {
        let filter = QuotesFilter { locale: "fr".to_string() };

        let result = filter.check("«Et moi aussi», dit l’Anglais.");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks with non-breaking spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(16, warnings[0].end);
    }

    #[test]
    fn test_filter_when_fr_and_no_warnings() {
        let filter = QuotesFilter { locale: "fr".to_string() };

        let result = filter.check("« Et moi aussi », dit l’Anglais.");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_filters_when_it_and_straight_quotation_marks() {
        let filter = QuotesFilter { locale: "it".to_string() };

        let result = filter.check("\"Anche a me\", ha detto la spagnola");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(12, warnings[0].end);
    }

    #[test]
    fn test_filters_when_it_and_english_quotation_marks() {
        let filter = QuotesFilter { locale: "it".to_string() };

        let result = filter.check("“Anche a me”, ha detto la spagnola");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(16, warnings[0].end);
    }

    #[test]
    fn test_filters_when_it_and_french_quotation_marks_with_spaces() {
        let filter = QuotesFilter { locale: "it".to_string() };

        let result = filter.check("« Anche a me », ha detto la spagnola");

        assert!(result.is_err());

        let warnings = result.err().unwrap();

        assert_eq!(1, warnings.len());
        assert_eq!("Please use french quotation marks without spaces.", warnings[0].message);
        assert_eq!(0, warnings[0].start);
        assert_eq!(18, warnings[0].end);
    }

    #[test]
    fn test_filter_when_it_and_no_warnings() {
        let filter = QuotesFilter { locale: "it".to_string() };

        let result = filter.check("«Anche a me», ha detto la spagnola");

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }
}

