use std::time::Duration;

use bulletin_board_client as bbclient;

#[test]
fn test_all() {
    bbclient::set_addr("localhost:7578");
    bbclient::set_timeout(Some(Duration::from_millis(100)));
    let data = vec![1f64, 2.];
    bbclient::post("title", "tag", data.clone().into()).unwrap();
    bbclient::relabel("title", None, Some("new_title"), Some("new_tag")).unwrap();
    dbg!(bbclient::view_board().unwrap());
    dbg!(bbclient::get_info("new_title", None).unwrap());

    #[cfg(not(feature = "dry_run"))]
    {
        let recv = bbclient::read("new_title", None, vec![])
            .unwrap()
            .pop()
            .unwrap();
        let restored: Vec<f64> = recv.try_into().unwrap();
        assert_eq!(data, restored);
    }
    bbclient::clear_revisions("title", None, vec![0]).unwrap();
    bbclient::remove("title", None).unwrap();
    bbclient::post("title", "tag", data.clone().into()).unwrap();
    bbclient::archive("acv", "title", None).unwrap();
    bbclient::load("acv").unwrap();
    bbclient::rename_archive("acv", "acv2").unwrap();
    bbclient::reset_server().unwrap();
    bbclient::restore("acv2").unwrap();
    bbclient::delete_archive("acv2").unwrap();
    bbclient::reset_server().unwrap();
    dbg!(bbclient::client_version());
    dbg!(bbclient::server_version().unwrap());
    dbg!(bbclient::status().unwrap());
    bbclient::clear_log().unwrap();
    dbg!(bbclient::log().unwrap());
    bbclient::terminate_server().unwrap();
}
