/**
* This script simply takes the contents of a ppm file and 
* formats it into a struct that makes it easy to work with.
* 
* For more information about the anatomy of PPM file, see: 
* https://paulbourke.net/dataformats/ppm/
* 
* A small excerpt from the description is as follows:
* 
* The first "line" is a magic PPM identifier, it can be 
* "P3" or "P6" (not including the double quotes!). The next
* line consists of the width and height of the image as ascii
* numbers. The last part of the header gives the maximum value
* of the colour components for the pixels, this allows the 
* format to describe more than single byte (0..255) colour 
* values. In addition to the above required lines, a comment
* can be placed anywhere with a "#" character, the comment 
* extends to the end of the line.
*
*/

use serde_json::json;

pub fn compile(s: &str) -> Result<String, &'static str> {
  // header 
  let mut ppm_identifier: &str = "";
  let mut width: i32 = -1;
  let mut height: i32 = -1;
  let mut max_val: i32 = -1;
  
  // actual data
  let mut pixels: Vec<Vec<u32>> = Vec::new();
  
  let mut captured_header:bool = false;
  let mut lines = s.lines();
  let mut pixel: Vec<u32> = Vec::new();

  while let Some(line) = lines.next() {
    let line = line.trim();
    
    // Skip line if it is a comment
    if line.len() > 0 && line.starts_with('#') {
      continue;
    }
      
    let mut parts = line.split_whitespace();
    while let Some(curr) = parts.next() {
      if captured_header == false {
        if ppm_identifier == "" {
          ppm_identifier = curr;
          if ppm_identifier != "P3" {
            return Err("Currently only PPM files with `P3` format are supporetd.")
          }
        } else if width == -1 {
          width = curr.parse::<i32>().map_err(|_| "Error: Unable to parse `width`: {}")?;
        } else if height == -1 {
          height = curr.parse::<i32>().map_err(|_| "Error: Unable to parse `height`")?;
        } else {
          max_val = curr.parse::<i32>().map_err(|_| "Error: Unable to parse `max_val`")?;
          captured_header = true;
        }
      } else {
        pixel.push(curr.parse::<u32>().map_err(|_| "Error: Unable to parse dimensions")?);
        if pixel.len() == 3 {
          pixels.push(pixel);
          pixel = Vec::new();
        }
      }
    }
  }
  
  if pixel.len() != 0 {
    return Err("Error: Malformed; missing RGB data");
  }
  
  let result_json = json!({
    "status": true,
    "data": {
      "header": {
        "ppm_identifier": ppm_identifier,
        "width": width,
        "height": height,
        "max_val": max_val
      },
      "data": {
        "pixels": pixels,
      }
    }
  });

  Ok(result_json.to_string())
}