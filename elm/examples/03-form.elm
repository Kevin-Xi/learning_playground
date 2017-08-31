import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onClick)
import String
import Char exposing (isUpper, isLower, isDigit)

main =
  Html.beginnerProgram
    { model = model
    , view = view
    , update = update
    }



-- MODEL


type alias Model =
  { name : String
  , password : String
  , passwordAgain : String
  , age: String
  , check: Bool
  }


model : Model
model =
  Model "" "" "" "" False



-- UPDATE


type Msg
    = Name String
    | Password String
    | PasswordAgain String
    | Age String
    | Check


update : Msg -> Model -> Model
update msg model =
  case msg of
    Name name ->
      { model | name = name, check = False }

    Password password ->
      { model | password = password, check = False }

    PasswordAgain password ->
      { model | passwordAgain = password, check = False }

    Age age ->
      { model | age = age, check = False }

    Check ->
      { model | check = True }


-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ input [ type_ "text", placeholder "Name", onInput Name ] []
    , input [ type_ "password", placeholder "Password", onInput Password ] []
    , input [ type_ "password", placeholder "Re-enter Password", onInput PasswordAgain ] []
    , input [ type_ "text", placeholder "Age", onInput Age ] []
    , button [ onClick Check ] [ text "Submit" ]
    , viewValidation model
    ]


viewValidation : Model -> Html msg
viewValidation model =
  if model.check then -- todo abstract this
    let
      (color, message) =
        case String.toInt model.age of
          Result.Err _ -> ("red", "age must be number")
          _ ->
          if String.length model.password <= 8 then
            ("red", "Password must longer than 8 chars")
          else if not (String.any isUpper model.password && String.any isLower model.password && String.any isDigit model.password) then
            ("red", "Password must contains upper, lower, digit chars")
          else if model.password == model.passwordAgain then
            ("green", "OK")
          else
            ("red", "Passwords do not match!")
    in
      div [ style [("color", color)] ] [ text message ]
  else
    div [] []