// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

pub trait TypeName {
    fn type_name(&self) -> &str;
}

pub trait TypeNameStatic {
    fn type_name_static() -> &'static str;
}
