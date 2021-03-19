#[cfg(test)]
mod tests {
    use bbqueue::{consts::*, BBBuffer};
    use futures::executor::block_on;

    #[test]
    fn test_read() {
        let bb: BBBuffer<U6> = BBBuffer::new();
        let (mut prod, mut cons) = bb.try_split().unwrap();

        {
            let mut grant = prod.grant_exact(4).unwrap();
            let buf = grant.buf();
            buf[0] = 0xDE;
            buf[1] = 0xAD;
            buf[2] = 0xC0;
            buf[3] = 0xDE;
            grant.commit(4);
        }

        let mut rx_buf = [0; 4];
        let result = block_on(cons.read_async(&mut rx_buf));

        assert_eq!(4, result.unwrap());
        assert_eq!(rx_buf[0], 0xDE);
        assert_eq!(rx_buf[1], 0xAD);
        assert_eq!(rx_buf[2], 0xC0);
        assert_eq!(rx_buf[3], 0xDE);
    }

    #[test]
    fn test_write() {
        let bb: BBBuffer<U6> = BBBuffer::new();
        let (mut prod, mut cons) = bb.try_split().unwrap();

        let result = block_on(prod.write_async(&[0xDE, 0xAD, 0xC0, 0xDE]));
        assert_eq!(4, result.unwrap());

        let grant = cons.read().unwrap();
        let rx_buf = grant.buf();
        assert_eq!(4, rx_buf.len());
        assert_eq!(rx_buf[0], 0xDE);
        assert_eq!(rx_buf[1], 0xAD);
        assert_eq!(rx_buf[2], 0xC0);
        assert_eq!(rx_buf[3], 0xDE);
    }
}
