pub fn inline_css() -> &'static str {
    r#"
    body {
        font-family: sans-serif;
        padding: 20px;
        background: #fafafa;
    }
    h1 {
        margin-bottom: 16px;
    }
    ul {
        list-style: none;
        padding-left: 0;
    }
    li {
        margin: 6px 0;
    }
    a {
        text-decoration: none;
        color: #0077cc;
    }
    a:hover {
        text-decoration: underline;
    }
    "#
}
