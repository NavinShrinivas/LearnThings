echo "Interactive shell, start a conversation!"

while :
do
    read INPUT
    case $INPUT in
        hello)
            echo "hey there!"
            ;;
        hi)
            echo "hey there!"
            ;;
        hey)
            echo "hey there!"
            ;;
        bye)
            echo "thenks for talking with me. :)"
            break;
            ;;
        *) echo "I don't understand thiss"
    esac
done
