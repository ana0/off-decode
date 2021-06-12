# off-decode
This is the secret to decode and decrypt the shared secrets from Off

To run this script will require a little bit, but not a lot of programming knowledge. I am confident you can figure it out together.

Prerequisites:

- Install git: [instructions](https://www.linode.com/docs/guides/how-to-install-git-on-linux-mac-and-windows/)
- Install rust: [instructions](https://www.rust-lang.org/tools/install)

I used rust version 1.47.0 to generate the secrets and test the decryption, other versions are untested, but may also work.

Here's how it works!

1. Open a terminal or command prompt in some convenient place, and download this code `git clone https://github.com/ana0/off-decode.git`

2. Place 170 of the secret images in the `off decode` folder that was just created. The secret images all begin with the prefix `Secret` and should not have been compressed or altered.

3. Move into the folder you just cloned down `cd off-decode`

4. Run the script! `cargo run`

It will take a little while to parse all the data. When finished and if successful, there will be a file in the folder called `secret.txt`

