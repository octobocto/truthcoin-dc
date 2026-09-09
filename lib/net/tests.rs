use std::{net::SocketAddr, time::Duration};

use super::{ALPHANET_SEED, Archive, DatabaseUnique, Net, Network, State};

#[test]
fn alphanet_names_the_seed_port() {
    assert_eq!(ALPHANET_SEED, ("seed.alpha.ecash.eu.com", 4013));
}

#[test]
fn seed_resolution_keeps_the_port() {
    let addrs = super::resolve_seed_addrs(("localhost", 4013)).unwrap();
    assert!(!addrs.is_empty());
    assert!(addrs.iter().all(|addr| addr.port() == 4013));
    assert!(addrs.iter().all(|addr| addr.ip().is_loopback()));
}

#[test]
fn seed_resolution_keeps_both_address_families() {
    let addrs: [SocketAddr; 2] = [
        "[::1]:4013".parse().unwrap(),
        "127.0.0.1:4013".parse().unwrap(),
    ];
    assert_eq!(super::resolve_seed_addrs(addrs.as_slice()).unwrap(), addrs);
}

#[test]
fn ipv4_node_connects_with_both_seed_families() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install the test TLS provider");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let dir = tempfile::tempdir().unwrap();
        let mut options = heed::EnvOpenOptions::new();
        options
            .map_size(16 * 1024 * 1024)
            .max_dbs(State::NUM_DBS + Archive::NUM_DBS + Net::NUM_DBS);
        let env = unsafe { sneed::Env::open(&options, dir.path()) }.unwrap();
        let state = State::new(&env, None).unwrap();
        let archive = Archive::new(&env).unwrap();
        let (remote, _) =
            super::make_server_endpoint("127.0.0.1:0".parse().unwrap())
                .unwrap();
        let ipv4 = remote.local_addr().unwrap();
        let ipv6 = SocketAddr::new("::1".parse().unwrap(), ipv4.port());
        {
            let mut txn = env.write_txn().unwrap();
            let peers = DatabaseUnique::<
                heed::types::SerdeBincode<SocketAddr>,
                heed::types::Unit,
            >::create(&env, &mut txn, "known_peers")
            .unwrap();
            for addr in [ipv6, ipv4] {
                peers.put(&mut txn, &addr, &()).unwrap();
            }
            txn.commit().unwrap();
        }
        let (net, _events) = Net::new(
            &env,
            archive,
            Network::Regtest,
            state,
            "0.0.0.0:0".parse().unwrap(),
        )
        .unwrap();
        let connection = tokio::time::timeout(Duration::from_secs(3), async {
            remote.accept().await.unwrap().await.unwrap()
        })
        .await
        .unwrap();
        assert!(connection.remote_address().is_ipv4());
        let peers = net.get_active_peers();
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address, ipv4);
        net.server.close(0_u32.into(), b"test complete");
        remote.close(0_u32.into(), b"test complete");
    });
}
