pub fn inline_css() -> &'static str {
    r#"
    * {
        box-sizing: border-box;
    }
    body {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
                     Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
        margin: 0;
        padding: 20px;
        background: #f7f7f7;
        color: #333;
    }
    h1 {
        font-size: 28px;
        margin-bottom: 10px;
        color: #222;
    }
    .header {
        max-width: 720px;
        margin: 0 auto;
        display: flex;
        justify-content: space-between;
        align-items: baseline;
        gap: 12px;
    }
    .sort-bar {
        font-size: 13px;
        color: #666;
        display: flex;
        align-items: center;
        gap: 6px;
        white-space: nowrap;
    }
    .sort-bar a {
        color: #666;
        font-weight: 500;
    }
    .sort-bar a.active {
        color: #1a73e8;
        text-decoration: underline;
    }
    .sort-bar .sep {
        color: #bbb;
    }
    .sort-bar .order a {
        margin-left: 4px;
        font-size: 12px;
        color: #999;
    }
    .sort-bar .order a:hover {
        color: #1a73e8;
    }
    .container {
        max-width: 720px;
        margin: 0 auto;
    }
    ul {
        list-style: none;
        padding-left: 0;
        margin-top: 20px;
    }
    li {
        background: white;
        margin-bottom: 10px;
        padding: 12px 16px;
        border-radius: 8px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        border: 1px solid #eaeaea;
        transition: all 0.15s ease;
        font-size: 15px;
    }
    li:hover {
        background: #f0f7ff;
        border-color: #cfe3ff;
    }
    a {
        text-decoration: none;
        color: #1a73e8;
        font-weight: 500;
    }
    a:hover {
        text-decoration: underline;
    }
    .file-name {
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .file-size {
        font-size: 13px;
        color: #666;
    }
    footer {
        margin-top: 40px;
        font-size: 13px;
        text-align: center;
        color: #999;
    }
    "#
}
