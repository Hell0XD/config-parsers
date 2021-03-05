mod parsers;
mod types;

pub mod json;
pub mod xml;
pub mod dotenv;

#[cfg(test)]
mod tests {
    use crate::json::{self};
    use crate::xml::{self};
    use crate::dotenv::{self};

    #[test]
    fn test_dotenv() {
        let dotenv = r#"
            S3_BUCKET=YOURS3BUCKET
            SECRET_KEY="YOURSECRETKEYGOESHERE"
        "#;

        let result = dotenv::parser(dotenv);

        let mut env = std::collections::HashMap::new();

        env.insert("S3_BUCKET", "YOURS3BUCKET");
        env.insert("SECRET_KEY", "YOURSECRETKEYGOESHERE");

        assert_eq!(Ok(env), result);
    }

    #[test]
    fn test_json() {
        let json = r#"
        {
            "userId": 1,
            "id": 1,
            "title": "delectus aut autem",
            "completed": false
        }
        "#;

        let result = json::parse(json);

        let mut obj = std::collections::HashMap::new();
        obj.insert("userId", json::JsonValue::Number(1.0));
        obj.insert("id", json::JsonValue::Number(1.0));
        obj.insert("title", json::JsonValue::String("delectus aut autem"));
        obj.insert("completed", json::JsonValue::Boolean(false));

        assert_eq!(Ok(json::JsonValue::Object(obj)), result);
    }

    #[test]
    fn test_xml() {
        let xml = r#"
        <note>
            <to>Tove</to>
            <from>Jani</from>
            <heading>Reminder</heading>
            <body>Don't forget me this weekend!</body>
            <short foo="bar" />
        </note>
      "#;

        let result = xml::parse(xml);


        assert_eq!(Ok(vec![xml::Element{
            name: "note",
            attrs: vec![],
            children: vec![
                xml::XmlValue::Element(xml::Element{
                    name: "to",
                    attrs: vec![],
                    children: vec![xml::XmlValue::Text("Tove")]
                }),
                xml::XmlValue::Element(xml::Element{
                    name: "from",
                    attrs: vec![],
                    children: vec![xml::XmlValue::Text("Jani")]
                }),
                xml::XmlValue::Element(xml::Element{
                    name: "heading",
                    attrs: vec![],
                    children: vec![xml::XmlValue::Text("Reminder")]
                }),
                xml::XmlValue::Element(xml::Element{
                    name: "body",
                    attrs: vec![],
                    children: vec![xml::XmlValue::Text("Don't forget me this weekend!")]
                }),
                xml::XmlValue::Element(xml::Element{
                    name: "short",
                    attrs: vec![("foo", "bar")],
                    children: vec![]
                }),
            ]
        }]), result);
    }

    
}
