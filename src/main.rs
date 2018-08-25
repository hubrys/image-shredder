extern crate raster;

use raster::{Image};

#[derive(Clone, Copy)]
enum SliceDirection {
    Horizontal,
    Vertical
}

fn copy_slice(src:&mut Image, dst:&mut Image, in_slice:i32, out_slice:i32, slice_size:i32, direction:SliceDirection) -> Result<(), raster::error::RasterError> {
    let (in_x, in_y, out_x, out_y, width, height) = match direction {
        SliceDirection::Horizontal => (0, slice_size * in_slice, 0, slice_size * out_slice, src.width, slice_size),
        SliceDirection::Vertical => (slice_size * in_slice, 0, slice_size * out_slice, 0, slice_size, src.height)
    };

    for width_idx in 0..width {
        for height_idx in 0..height {
            let pixel = src.get_pixel(width_idx + in_x, height_idx + in_y)?;
            dst.set_pixel(width_idx + out_x, height_idx + out_y, pixel)?;
        }
    }

    Ok(())
}

fn slice(src:&mut Image, dst:&mut Image, sections:i32, slices_per_section:i32, direction:SliceDirection) -> Result<(), raster::error::RasterError> {
    let slice_count = sections * slices_per_section;
    let slice_size = match direction {
        SliceDirection::Horizontal => src.height / slice_count,
        SliceDirection::Vertical => src.width / slice_count
    };

    for slice_idx in 0..slice_count as i32 {
        let out_section = slice_idx % sections;
        let out_offset = slice_idx / sections;
        let out_slice_idx = out_section * slices_per_section + out_offset;

        copy_slice(src, dst, slice_idx, out_slice_idx, slice_size, direction)?;
    }
    Ok(())
}

fn main() {
    let mut original_image = raster::open("tests/in/horses.jpg").unwrap();
    let mut output_image = Image::blank(original_image.width, original_image.height);

    slice(&mut original_image, &mut output_image, 2, 40, SliceDirection::Vertical).unwrap();

    let tmp = output_image;
    output_image = original_image;
    original_image = tmp;

    slice(&mut original_image, &mut output_image, 2, 40, SliceDirection::Horizontal).unwrap();

    raster::save(&output_image, "tests/out/horses.jpg").unwrap();
}
