#[tokio::test]
async fn organization_index_test() -> httpc_test::Result<()> {
    let http_client = httpc_test::new_client("http://localhost:3000")?;

    let response = http_client.do_get("/organization").await?;

    let status = response.status();

    println!("status: {:?}", status);
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    response.print().await?;

    assert_eq!(status, 200);

    Ok(())
}
