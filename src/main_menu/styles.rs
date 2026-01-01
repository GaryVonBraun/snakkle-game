use bevy::prelude::*;

pub fn main_menu_img_bundle(image: Handle<Image>) -> impl Bundle {
    (
        ImageNode {
            image: image,
            ..default()
        },
        Node {
            width: Val::Px(64.),
            height: Val::Px(64.),
            margin: UiRect::all(Val::Px(8.)),
            ..default()
        },
    )
}
