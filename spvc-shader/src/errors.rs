use rspirv;

error_chain! {
    foreign_links {
        Rspirv(rspirv::mr::Error);
    }

    errors {
        MissingType {
        }
    }
}
