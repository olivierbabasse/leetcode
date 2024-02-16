//! <https://leetcode.com/problems/encode-and-decode-tinyurl/>

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, long_url: String) -> String {
        long_url
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        short_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let url = "https://leetcode.com/problems/design-tinyurl";
        let obj = Codec::new();
        let tiny = obj.encode(url.into());
        assert_eq!(obj.decode(tiny), url.to_string());
    }
}
