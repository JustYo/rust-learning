fn main() {
    let an_integer = 1u32;
    let a_boolean = true;

    let copied_integer = an_integer;

    println!("{:?}, {:?}, {:?}", an_integer, a_boolean, copied_integer);

    let _unused_variable = 3u128;

    let _variable = 3u32;

    //mutable variables
    let _immutable_variable = 1;
    let mut mutable_variable = 2;

    println!("Before mutation: {}", mutable_variable);

    mutable_variable = 3;

    println!("After mutation: {}", mutable_variable);

    //Error
    // _immutable_variable = 43;

    //scope and shadowing
    let long_live_binding = 1;

    // This is a block and has a smaller scope than the main function
    {
        let shrt_lived_binding = 2;
        println!("inner short: {}", shrt_lived_binding);
    }
    // println!("inner short: {}", shrt_lived_binding); error

    println!("outer long: {}", long_live_binding);

    //shadowing
    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abd";
        println!("shadowed inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    //Declare first
    let a_binding;
    {
        let x = 2;

        a_binding = x * x;
    }
    println!("{}", a_binding);

    //Freezing
    let mut _mutable_integer = 7;

    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50; not mutable anymore
    }
    _mutable_integer = 3;
    println!("{}", _mutable_integer)
}
// Dockerfile examples :

/*
FROM

# Open containers labels (See: https://github.com/opencontainers/image-spec/blob/master/annotations.md)
LABEL org.label-schema.vendor="" \
  org.opencontainers.image.authors="" \
  org.opencontainers.image.created="{% created %}" \
  org.opencontainers.image.description="" \
  org.opencontainers.image.documentation="" \
  org.opencontainers.image.source="" \
  org.opencontainers.image.title="" \
  org.opencontainers.image.url="" \
  org.opencontainers.image.vendor="" \
  org.opencontainers.image.version=""

# Add a new user "user" with user id 8877
RUN useradd -u 8877 user

#install dependencies
RUN apt-get update && apt-get install -y python3-pip

# Change to non-root privilege
USER user

CMD ["/bin/bash"]
 */
