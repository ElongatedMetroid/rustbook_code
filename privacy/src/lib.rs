mod outermost {
    // outermost::middle_function() is the only function that will not give an error
    // this is because even though outermost is private middle_function is public
    // and since try_me() is in the same module as outermost it is aloud to access
    pub fn middle_function() {

    }
    fn middle_secret_function() {

    }

    // the inside module is not public and it is inside outermost (inside a different module than try_me())
    mod inside {
        pub fn inner_function() {

        }

        fn secret_function() {

        }
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
