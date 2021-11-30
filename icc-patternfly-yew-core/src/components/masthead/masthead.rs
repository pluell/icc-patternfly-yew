use yew::{
    prelude::*,
};

use crate::{Breakpoints, InsetModifer};


#[derive(Clone, PartialEq)]
pub enum MastheadBackgroundColor
{
    Dark,
    Light,
    Light200,
}

#[derive(Clone, PartialEq)]
pub enum MastheadDisplayStyles
{
    Inline,
    Stack,
}

#[derive(Clone, PartialEq)]
pub struct MastheadDisplay
{
    breakpoint: Breakpoints,
    style: MastheadDisplayStyles,
}

impl MastheadDisplay
{
    pub fn get_class(&self) -> String
    {
        let style = match self.style {
            MastheadDisplayStyles::Inline => "inline",
            MastheadDisplayStyles::Stack => "stack"
        };

        let breakpoint = self.breakpoint.get_class_appendix();

        format!("pf-m-display-{}{}", style, breakpoint)
    }
}


pub struct Masthead
{
    props: MastheadProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MastheadProps
{
    /** Content rendered inside of the masthead */
    #[prop_or_default]
    pub children: Children,
    /** Additional classes added to the masthead */
    #[prop_or_default]
    pub class_name: String,
    /** Background theme color of the masthead */
    #[prop_or(MastheadBackgroundColor::Dark)]
    pub background_color: MastheadBackgroundColor,
    /** Display type at various breakpoints */
    #[prop_or_default]
    pub display: Vec<MastheadDisplay>,
    /** Insets at various breakpoints */
    #[prop_or_default]
    pub inset: Vec<InsetModifer>,
}

impl Component for Masthead
{
    type Message = ();
    type Properties = MastheadProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self
    {
        Self {
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            
            true
        }
        else
        {
            false
        }
    }

    /// Called everytime when messages are received
    fn update(&mut self, _: Self::Message) -> ShouldRender
    {
        true
    }

    fn view(&self) -> Html
    {
        html!{
            <header
                class=classes!(
                    "pf-c-masthead",
                    self.props.display.iter().map(|disp_mod| disp_mod.get_class()).collect::<Vec<String>>(),
                    self.props.inset.iter().map(|inset_mod| inset_mod.get_class()).collect::<Vec<String>>(),
                    if self.props.background_color == MastheadBackgroundColor::Light {"pf-m-light"} else {""},
                    if self.props.background_color == MastheadBackgroundColor::Light200 {"pf-m-light-200"} else {""},
                    self.props.class_name.clone(),
                )
                // {...props}
            >
                {for self.props.children.iter()}
            </header>
        }
    }
}
