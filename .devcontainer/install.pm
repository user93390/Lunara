#  Copyright 2026 seasnail1
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#

package install;
use strict;
use warnings FATAL => 'all';

my @packages = (
    "make",
    "docker",
    "git",
    "rustup",
    "unzip"
);

my @optional_packages = (
    "fish",
    "helix",
);


# Change if you want.
my $optional = 1;

sub install_required {
    print("Installing packages...\n");


    system("curl -fsSL https://bun.com/install | bash");

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
    system("cargo install fd-find");
    system("cargo install bottom --locked");

    system("echo '[charm]
    name=Charm
    baseurl=https://repo.charm.sh/yum/
    enabled=1
    gpgcheck=1
    gpgkey=https://repo.charm.sh/yum/gpg.key' | sudo tee /etc/yum.repos.d/charm.repo
    sudo yum install glow"
    );

    for my $optional_package (@optional_packages) {
        install($optional_package);
    }
}

sub install {
    my ($arg) = @_;
    system("sudo dnf install " . $arg);
    print("Installed " . $arg . "\n");
}

1;