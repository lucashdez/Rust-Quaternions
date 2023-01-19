use quaternion::*;

mod quaternion;
fn main()
{
    let q1 = Quaternion::new(0f32, 10f32, 0f32, 0f32);
    let q2 = Quaternion::new(0f32, 5f32, 0f32, 0f32);
    println!("q1: {}", q1);
    println!("q2: {}", q2);
    // Vector de rotacion
    let n1 = (0f32, 0f32, 1f32);
    let n2 = (0f32, 0f32, 1f32);

    Quaternion::add(&q1, &q2);

    Quaternion::product(&q1, &q2);

    // NOTE : Imprimir con plotters cada operacion

    // q1r = rotate input q1
    // q1c = conjugate q1
    //
    // q2r = rotation q2
    // q2c = conjugate q2
    //
    // i1 = multquat(q1,q1r)
    // o1 = multquat(i1, q1c)
    //
    // i2 = multquat(q1r, q2r)
    // i2 = multquat(i2, q2)
    // i2 = multquat(i2, q2c)
    // i2 = multquat(i2, q1c)
    // o2 = add(o1, i2)

    // imprimir visual con plotters
}
