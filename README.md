# `Gen3-Charset`
> This crate provides Pokemon generation 3 character set

## Maintenance 
This crate is **experimental at best** and is not likely to receive updates. However if there are bugs 
and you send a PR, I will consider merging it if it fixes the problem. **USE AT YOUR OWN RISK!**


## Usage 
> (International: EN, DE, IT, FR)
```rust
    use gen3_charset::{PkString, Intl};

    fn parse_pkstring_intl() {
        let bytes = [0xBCu8, 0xCF, 0xBE, 0xC3, 0xFF, 0xFF, 0xFF];

        // you will need to provide internal buffer type
        // you will also need to provide the language at compile time
        let s = PkString::<Intl, [u8; 7]>::from(bytes);
        assert_eq!(&format!("{}", s), "BUDI");
        println!("{}", s);
        println!("{:?}", s);
    }
```
> (Jpn)
```rust
    use gen3_charset::{PkString, Jpn};

    fn parse_pkstring_jpn() {
        let bytes = [112u8, 142, 139, 123, 83, 255, 0, 8, 76, 125];
        
        // you will need to provide internal buffer type
        // you will also need to provide the language at compile time
        let s = PkString::<Jpn, [u8; 10]>::from(bytes);
        assert_eq!(&format!("{}", s), "ミズゴロウ");
        println!("{}", s);
        println!("{:?}", s);
    }
```

## License
gen3-charset is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

## Donations
> For Singapore based users, you can donate using paylah!

<img src="./paylah.png" width="250">