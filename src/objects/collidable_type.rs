// *************************************************************************
//
// Copyright (c) 2025 Andrei Gramakov. All rights reserved.
//
// This file is licensed under the terms of the MIT license.
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://agramakov.me
// e-mail:  mail@agramakov.me
//
// *************************************************************************

pub enum CollidableType {
    Unknown,
    SolidObstacle,
    Player,
    Collectable,
}

impl CollidableType {
    pub fn to_int(&self) -> u32 {
        match self {
            CollidableType::SolidObstacle => 1,
            CollidableType::Player => 2,
            CollidableType::Collectable => 3,
            _ => 0, // Return 0 for unknown values
        }
    }

    pub fn from_int(value: u32) -> CollidableType {
        match value {
            1 => CollidableType::SolidObstacle,
            2 => CollidableType::Player,
            3 => CollidableType::Collectable,
            _ => CollidableType::Unknown, // Return None for invalid values
        }
    }
}
