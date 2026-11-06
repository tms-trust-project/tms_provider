{ ... }:
{
  config.perSystem = { config, pkgs, ... }:
    let
      image = pkgs.dockerTools.buildLayeredImage {
        name = "tms-provider";
        tag = config.tms-provider.git_commit;
        contents = [ config.packages.default ];
        config.Cmd = [ "${config.packages.default}/bin/tms-provider" ];
      };
    in
    {
      config.packages = {
        inherit image;
      };
    };
}
