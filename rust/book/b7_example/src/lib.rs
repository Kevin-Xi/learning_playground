mod outermost {
    pub fn middle_function() {}

    // private, can only access by current module(outermost) and its children(inside)
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            // need import ...
            use outermost::middle_function;
            middle_function();
            // ... or refer from root
            ::outermost::middle_secret_function();    
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
