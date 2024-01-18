fn examples_sends() -> Vec<String> {
    let mut examples = vec![
        "da3ed1efda82824cb24ea081ef2a8f532a7dd9cd1ebc5efa873498c3958c864e", //0  Classic send - 501 JPGOLD
        "585f50f12288cd9044705483672fbbddb71dff8198b390b40ab3de30db0a88dd", //1  Classic send - 0.2 XCP
        "d9bdf3b63b8283744762713aa72621822e9562e5823f44366edde017e00d3da8", //2  Enhanced send - 497.83516484 XCP
        "11ae7493f5a8ef8691b391a95dd9e649afb21f449c5de67d4f6a02fce72d3645", //3  Enhanced send with memo
        "b55c5745a6106314e41f68c8f4f96afcdf4f637ecbf2165cb454983e28843e3a", //4  DEX order
        "1c10c283e7aa2baf8977fa6c15556f4934ac7238c65f47fa47cd836918d5546b", //5  DEX order with BTC
        "be13ee06eee97c44c550c6297d6845f8963ef3461b4b9668207f65273d8aaf1b", //6  Btcpay (after order match)
        "49896f6115956c2f9baddb9cf0991ba114d010bfffcc73f1d3ddb8e4267aa272", //7  Dispenser - JPJA
        "8aad0368ee12b380d77437a5bfc9a9a19a4b6e96d0127e52b92d4bf44736cb44", //8  Dispenser - LOCHNESS - separate address
        "e5e9f6a63ede5315994cf2d8a5f8fe760f1f37f6261e5fbb1263bed54114768a", //9  Issuance - OLGA
        "34da6ecf10c66ed659054aa6c71900c807875cb57b96abea4cee4f7a831ed690", //10 Issuance - lock OLGA
        "541e640fbb527c35e0ee32d724efa4a5506c4c52acfba1ebc3b45949780c08a8", //11 Issuance - transfer SALVATION ownership
        "21c2cd5b369c2e7a350bf92ad43c31e5abb0aa85ccba11368b08f9f4abb8e0af", //12 Broadcast - jpja.net
        "9d356c8c455e0be7381c6f35413d0b45c00947797f9df193c583337ac11e1c24", //13 Broadcast - Chinese
        "627ae48d6b4cffb2ea734be1016dedef4cee3f8ffefaea5602dd58c696de6b74", //14 Broadcast - OLGA image
        "756df60b4a97ac41912a03b95ea4b027ed9d9d07fce3fc0a2de8744e6fc5cd94", //15 Dividend - JPBULL to JPBEAR holders
        "56afd17f57a2815e86b324465ac264d1dee2bcedd847361754ef49887d116ba0", //16 Sweep
        "793566ef1644a14c2658aed6b3c2df41bc519941f121f9cff82825f48911e451", //17 Subasset issuance
        "940b4fede6ca11446c60e4f89dea1c38b7169b7c8fd1805e85eedc5b448a4f0d", //18 Enhanced send to bech32 addr
        "ca3ffd78d333969d333686563080a76830ce4df7c771e47e482317091ef069f4", //19 Enhanced send to multisig addr
        "31a96a3bd86600b4af3c81bc960b15e89e506f855e93fbbda6f701963b1936ac", //20 Issuance with STAMP image
        "549a5cc4bc189c800f0f9ea01068e8a7fd987c7dadb40c0b6a224d489ed070cc", //21 Issuance with FILE in p2wsh outputs (cip33)
        "d36f1176ab64d9fc3898ca97822b9f900f11ce4da80ed3f474b75d7a1ebdc597", //22 Destruction
    ];
    examples.into_iter().map(|s| s.to_string()).collect()
}
