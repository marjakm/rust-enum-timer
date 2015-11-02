/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Mattis Marjak (mattis.marjak@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_export]
macro_rules! et_create_enum_timer {
    ( $storage_name:ident;
        $(#[$attr:meta])*
        pub enum $enum_name:ident {
            $($var:ident),*
        }
    ) => {
        // Create enum itself
        $(#[$attr])*
        pub enum $enum_name {
            $($var),*
        }

        // Create timer storage
        #[allow(non_snake_case)]
        pub struct $storage_name {
            $($var : Option<SteadyTime>,)*
            idx: Option<$enum_name>
        }

        // Implement TimerStorage trait on storage
        impl TimerStorage<$enum_name> for $storage_name {
            fn new() -> Self {
                $storage_name {
                    $($var : None,)*
                    idx: None
                }
            }

            fn clear(&mut self, variant: &$enum_name) {
                match *variant {
                    $($enum_name::$var => {self.$var  = None;}),*
                }
            }

            fn set(&mut self, variant: &$enum_name, when: SteadyTime) {
                match *variant {
                    $($enum_name::$var => {self.$var  = Some(when);}),*
                }
            }

            fn reset_next(&mut self) {
                self.idx = Some(_et_first!($enum_name, $($var),*));
            }

            fn next(&mut self) -> Option<TimerEvent<$enum_name>> {
                loop {
                    _et_match_statement!( self, $enum_name > $($var),* );
                };
            }

        }
    };
}

#[macro_export]
macro_rules! _et_match_statement {
    ( $slf:ident, $enum_name:ident > $first:ident, $second:ident) => {
        _et_match_statement!( $slf, $enum_name < ($first, $second), ($second, None) )
    };
    ( $slf:ident, $enum_name:ident > $first:ident, $second:ident, $($rest:ident),*) => {
        _et_match_statement!( $slf, $enum_name > $second, $($rest),* ; ($first, $second) )
    };
    ( $slf:ident, $enum_name:ident > $first:ident, $second:ident, $($rest:ident),* ; $($completed:tt),*) => {
        _et_match_statement!( $slf, $enum_name > $second, $($rest),* ; $($completed),* , ($first, $second) )
    };
    ( $slf:ident, $enum_name:ident > $first:ident, $second:ident ; $($completed:tt),*)   => {
        _et_match_statement!( $slf, $enum_name < $($completed),* , ($first, $second), ($second, None) )
    };
    ( $slf:ident, $enum_name:ident < $(($cur_state:ident, $next_state:ident)),* )   => {
        match $slf.idx {
            $(
                Some($enum_name::$cur_state) => {
                    $slf.idx = _et_some_or_none!($enum_name, $next_state);
                    if $slf.$cur_state.is_some() {
                        return Some(TimerEvent {variant: $enum_name::$cur_state, when: $slf.$cur_state.as_ref().unwrap().clone()})
                    }
                },
            )*
            _ => {
                $slf.reset_next();
                return None
            }
        };
    };
}

#[macro_export]
macro_rules! _et_first {
    ($enum_name:ident, $first:ident, $($rest:ident),*) => {$enum_name::$first}
}

#[macro_export]
macro_rules! _et_some_or_none {
    ($enum_name:ident, None) => {None};
    ($enum_name:ident, $next_state:ident) => {Some($enum_name::$next_state)};
}
