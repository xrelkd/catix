{ name
, version
, dockerTools
, catix
, buildEnv
, ...
}:

dockerTools.buildImage {
  inherit name;
  tag = "v${version}";

  copyToRoot = buildEnv {
    name = "image-root";
    paths = [ catix ];
    pathsToLink = [ "/bin" ];
  };

  config = {
    Entrypoint = [ "${catix}/bin/catix" ];
  };
}
