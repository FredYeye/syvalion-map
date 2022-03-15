mod syvalion;

fn main()
{
    let syvalion = syvalion::Syvalion::default();

    //generate maps for the 5 basic maps
    for x in 1 ..= 5
    {
        syvalion.print_map_image(x);
    }
}
