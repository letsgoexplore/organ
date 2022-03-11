use rug::{integer::Order, Integer};
use openssl::{bn::{BigNum, BigNumContext}, ec::{EcPoint, EcPointRef, PointConversionForm}};
use num_traits::Signed;

pub fn new_big_num_context() -> BigNumContext {
    BigNumContext::new().unwrap()
}

pub fn get_g(params: &crate::config::ProtocolParams) -> EcPoint {
    params.group.as_ref().unwrap().generator().to_owned(&params.group.as_ref().unwrap()).unwrap()
}

pub fn get_h(params: &crate::config::ProtocolParams) -> EcPoint {
    params.group.as_ref().unwrap().generator().to_owned(&params.group.as_ref().unwrap()).unwrap()
}

pub fn add(params: &crate::config::ProtocolParams, a: &EcPointRef, b: &EcPointRef) -> EcPoint {
    let mut point = EcPoint::new(&params.group.as_ref().unwrap()).unwrap();
    point.add(&params.group.as_ref().unwrap(), a, b, &mut new_big_num_context()).unwrap();
    point
}

pub fn mul(params: &crate::config::ProtocolParams, a: &EcPointRef, b: &Integer) -> EcPoint {
    let mut point = EcPoint::new(&params.group.as_ref().unwrap()).unwrap();
    point.mul(&params.group.as_ref().unwrap(), a, &to_scalar(b), &mut new_big_num_context()).unwrap();
    point
}

pub fn to_bytes(params: &crate::config::ProtocolParams, a: &EcPointRef) -> Vec<u8> {
    a.to_bytes(&params.group.as_ref().unwrap(), PointConversionForm::UNCOMPRESSED, &mut new_big_num_context()).unwrap()
}

pub fn from_bytes(params: &crate::config::ProtocolParams, buf: &[u8]) -> EcPoint {
    EcPoint::from_bytes(&params.group.as_ref().unwrap(), buf, &mut new_big_num_context()).unwrap()
}

pub fn get_order(params: &crate::config::ProtocolParams) -> Integer {
    let mut ret = BigNum::new().unwrap();
    params.group.as_ref().unwrap().order(&mut ret, &mut new_big_num_context()).unwrap();
    Integer::from_digits(&ret.to_vec(), Order::Msf)
}

pub fn to_scalar(x: &Integer) -> BigNum {
    /*
    let order: Integer = get_order();
    let (_quotient, rem) = x.clone().div_rem_euc(order);
    */
    let bn = BigNum::from_slice(&x.to_digits::<u8>(Order::Msf)).unwrap();
    if <Integer as Signed>::is_negative(x) {
        -bn
    } else {
        bn
    }
}
