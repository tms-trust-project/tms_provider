# For all the options available for this module, please run
# 
#    nix run .#docs-serve
#
{
  tms-provider = {
    version = "0.1.0";
    jwt_issuers = [
      "https://dev.develop.tapis.io/v3/tokens"
      "http://localhost:8080"
    ];
    RUST_LOG = "debug";
  };
}
