{
  inputs.fia.url = github:poollovernathan/fia;
  inputs.tree-sitter-lit.url = github:poollovernathan/tree-sitter-lit;
  inputs.tree-sitter-lit.flake = false;
  outputs = { self, fia, tree-sitter-lit }: {
    packages = fia.lib.perSystem (pkgs: {
      default = fia.lib.crossCompile' {
        inherit pkgs;
        src = ./.;
        target = null;
      };
    });
  };
}
