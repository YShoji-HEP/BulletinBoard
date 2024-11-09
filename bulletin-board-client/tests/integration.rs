use bulletin_board_client as bbclient;

#[test]
fn post_read() {
    let data = vec![1f64, 2.];
    bbclient::post("title", "tag", data.clone().into()).unwrap();
    bbclient::relabel("title", None, Some("new_title"), Some("new_tag")).unwrap();
    let recv = bbclient::read("new_title", None, vec![]).unwrap().pop().unwrap();
    let restored: Vec<f64> = recv.try_into().unwrap();
    assert_eq!(data, restored);
}
