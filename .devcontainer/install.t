my @packages = (
    "just",
    "docker",
    "git",
    "rustup"
);

my @optional_packages = (
    "fish",
    "helix",
);

my $optional = 1;

sub install_required {
    print("Installing packages...\n");

    for my $package (@packages) {
        install($package);
    }

    if (!$optional) {
        return;
    }

    install_optional();
}

sub install_optional {
    print("installing option dependencies... \n");

    system("cargo install ripgrep");
    system("cargo install fd");
    system("cargo install bottom --locked");

    for my $optional_package (@optional_packages) {
        install($optional_package);
    }
}

sub install {
    my ($arg) = @_;
    system("sudo dnf install " . $arg);
    print("Installed " . $arg . "\n");
}
