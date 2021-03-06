// This file is part of Moonfire NVR, a security camera digital video recorder.
// Copyright (C) 2016 Scott Lamb <slamb@slamb.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// In addition, as a special exception, the copyright holders give
// permission to link the code of portions of this program with the
// OpenSSL library under certain conditions as described in each
// individual source file, and distribute linked combinations including
// the two.
//
// You must obey the GNU General Public License in all respects for all
// of the code used other than OpenSSL. If you modify file(s) with this
// exception, you may extend this exception to your version of the
// file(s), but you are not obligated to do so. If you do not wish to do
// so, delete this exception statement from your version. If you delete
// this exception statement from all source files in the program, then
// also delete it here.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use db;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TopLevel<'a> {
    pub time_zone_name: &'a str,

    // Use a custom serializer which presents the map's values as a sequence and includes the
    // "days" attribute or not, according to the bool in the tuple.
    #[serde(serialize_with = "TopLevel::serialize_cameras")]
    pub cameras: (&'a BTreeMap<i32, db::Camera>, bool),
}

/// JSON serialization wrapper for a single camera when processing `/cameras/` and
/// `/cameras/<uuid>/`. See `design/api.md` for details.
#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Camera<'a> {
    pub uuid: Uuid,
    pub short_name: &'a str,
    pub description: &'a str,
    pub retain_bytes: i64,
    pub min_start_time_90k: Option<i64>,
    pub max_end_time_90k: Option<i64>,
    pub total_duration_90k: i64,
    pub total_sample_file_bytes: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "Camera::serialize_days")]
    pub days: Option<&'a BTreeMap<db::CameraDayKey, db::CameraDayValue>>,
}

impl<'a> Camera<'a> {
    pub fn new(c: &'a db::Camera, include_days: bool) -> Self {
        Camera{
            uuid: c.uuid,
            short_name: &c.short_name,
            description: &c.description,
            retain_bytes: c.retain_bytes,
            min_start_time_90k: c.range.as_ref().map(|r| r.start.0),
            max_end_time_90k: c.range.as_ref().map(|r| r.end.0),
            total_duration_90k: c.duration.0,
            total_sample_file_bytes: c.sample_file_bytes,
            days: if include_days { Some(&c.days) } else { None },
        }
    }

    fn serialize_days<S>(days: &Option<&BTreeMap<db::CameraDayKey, db::CameraDayValue>>,
                         serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let days = match *days {
            Some(d) => d,
            None => return serializer.serialize_none(),
        };
        let mut map = serializer.serialize_map(Some(days.len()))?;
        for (k, v) in days {
            map.serialize_key(k.as_ref())?;
            let bounds = k.bounds();
            map.serialize_value(&CameraDayValue{
                start_time_90k: bounds.start.0,
                end_time_90k: bounds.end.0,
                total_duration_90k: v.duration.0,
            })?;
        }
        map.end()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
struct CameraDayValue {
    pub start_time_90k: i64,
    pub end_time_90k: i64,
    pub total_duration_90k: i64,
}

impl<'a> TopLevel<'a> {
    /// Serializes cameras as a list (rather than a map), optionally including the `days` field.
    fn serialize_cameras<S>(cameras: &(&BTreeMap<i32, db::Camera>, bool),
                            serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(cameras.0.len()))?;
        for c in cameras.0.values() {
            seq.serialize_element(&Camera::new(c, cameras.1))?;
        }
        seq.end()
    }
}

#[derive(Debug, Serialize)]
pub struct ListRecordings {
    pub recordings: Vec<Recording>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Recording {
    pub start_time_90k: i64,
    pub end_time_90k: i64,
    pub sample_file_bytes: i64,
    pub video_samples: i64,
    pub video_sample_entry_sha1: String,
    pub start_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_id: Option<i32>,
    pub video_sample_entry_width: u16,
    pub video_sample_entry_height: u16,
}
