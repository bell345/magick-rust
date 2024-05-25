/*
 * Copyright 2024 5ohue
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ResolutionType {
    Undefined = bindings::ResolutionType_UndefinedResolution,
    PixelsPerInch = bindings::ResolutionType_PixelsPerInchResolution,
    PixelsPerCentimeter = bindings::ResolutionType_PixelsPerCentimeterResolution,
}

impl Default for ResolutionType {
    fn default() -> Self {
        return ResolutionType::Undefined;
    }
}

impl From<ResolutionType> for bindings::ResolutionType {
    fn from(value: ResolutionType) -> Self {
        return value as bindings::ResolutionType;
    }
}

impl From<bindings::ResolutionType> for ResolutionType {
    fn from(value: bindings::ResolutionType) -> Self {
        /*
         * SAFETY:
         *
         * `ResolutionType` has the same repr as `bindings::ResolutionType` - u32
         *
         * If `value` is less than PixelsPerCentimeter than it is in the vaild range and can be safely
         * reinterpreted as `ResolutionType`
         */
        if value <= bindings::ResolutionType_PixelsPerCentimeterResolution {
            return unsafe { std::mem::transmute(value) };
        }
        return ResolutionType::default();
    }
}
