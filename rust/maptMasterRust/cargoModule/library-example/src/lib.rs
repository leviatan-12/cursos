fn suma(a:u16, b:u16) -> u16 {
    a+b
}


#[cfg(test)]
mod test {
    use super::suma;
    #[test]
    fn prueba_suma() {
        assert_eq!(suma(1,1),2);
    }
}
