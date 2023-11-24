#!/usr/bin/env bash

# Copyright 2023 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0
import argparse
import re
import os
import subprocess
import sys


def main(argv=[], prog_name=''):
    parser = argparse.ArgumentParser(prog='CI script',
                                     description='Executes all CI tests')
    args = parser.parse_args()

    # Points to Parsec's Unix Domain Socket on the CI
    os.environ['PARSEC_SERVICE_ENDPOINT'] = "unix:/tmp/parsec.sock"
    os.environ['RUST_LOG'] = "error"

    #########
    # Build #
    #########
    subprocess.check_output('RUST_BACKTRACE=1 cargo build', shell=True)
    subprocess.check_output('RUST_BACKTRACE=1 cargo build --features spiffe-auth', shell=True)

    #################
    # Static checks #
    #################
    # On native target clippy or fmt might not be available.
    if subprocess.run("cargo fmt -h", shell=True).returncode == 0:
        output = subprocess.check_output("cargo fmt --all -- --check", shell=True).decode()
        print(output)

    if subprocess.run("cargo clippy -h", shell=True).returncode == 0:
        cmd = "cargo clippy --all-targets -- -D clippy::all -D clippy::cargo"
        output = subprocess.check_output(cmd, shell=True).decode()
        print(output)

    #############
    # CLI tests #
    #############
    output = subprocess.check_output('./target/debug/parsec-tool --help', shell=True).decode()
    print(output)

    cmd = 'PARSEC_TOOL="./target/debug/parsec-tool" tests/parsec-cli-tests.sh -d'
    output = subprocess.check_output(cmd, shell=True).decode()
    print(output)

    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:], sys.argv[0]))

