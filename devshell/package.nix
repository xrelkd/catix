{ name
, version
, lib
, rustPlatform
, protobuf
, installShellFiles
}:

rustPlatform.buildRustPackage {
  pname = name;
  inherit version;

  src = lib.cleanSource ./..;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [
    protobuf

    installShellFiles
  ];

  postInstall = ''
    for cmd in catix; do
      installShellCompletion --cmd $cmd \
        --bash <($out/bin/$cmd completions bash) \
        --fish <($out/bin/$cmd completions fish) \
        --zsh  <($out/bin/$cmd completions zsh)
    done
  '';

  meta = with lib; {
    description = "Nix binary cache proxy service";
    homepage = "https://github.com/xrelkd/catix";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [ xrelkd ];
    mainProgram = "catix";
  };
}
