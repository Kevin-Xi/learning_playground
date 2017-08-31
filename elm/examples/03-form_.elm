import Html exposing (Html, div, input, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)

main =
    Html.beginnerProgram
        {
            model = model,
            view = view,
            update = update
        }

type alias Model =
    {
        name: String,
        pwd: String,
        pwdAgain: String
    }

model : Model
model =
    Model "" "" ""

type Msg
    = InputName String
    | InputPwd String
    | InputPwdAgain String

update : Msg -> Model -> Model
update msg model =
    case msg of
        InputName input -> { model | name = input}
        InputPwd input -> { model | pwd = input}
        InputPwdAgain input -> { model | pwdAgain = input}


view : Model -> Html Msg
view model =
    div []
        [
            input [ placeholder "name", onInput InputName ] [],
            input [ placeholder "pwd", type_ "password", onInput InputPwd ] [],
            input [ placeholder "pwdAgain", type_ "password", onInput InputPwdAgain ] [],
            div [] [ text (if model.pwd == model.pwdAgain then "matched!" else "not matched!")]
        ]