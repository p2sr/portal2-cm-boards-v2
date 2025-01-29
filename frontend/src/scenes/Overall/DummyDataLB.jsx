const avatarLink = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAALgAAAC4CAMAAABn7db1AAAAhFBMVEUiIiIhISEkJCQlJSUeHh5iYmLh4eHe3t7j4+MoKCjb29sbGxvm5uYqKioZGRkXFxcvLy/t7e3V1dWBgYFdXV2zs7NnZ2c/Pz9QUFDNzc1SUlISEhI1NTWXl5eLi4vAwMB8fHxxcXH29vY8PDykpKSRkZG3t7erq6udnZ1ISEgKCgp/f39EOWOzAAAJr0lEQVR4nO2da2/iPBOGfZjG58Q2KRRIS0vPu////70z2SVUrdA+b9qEIOX+UFUVMhfDeE42hd2s6np5dVFa1vXqhq1EZBK4gAuR4CBZFCtWNxEkMCEvRIIhbmxqVkuo5LnN+P9JViBrtjQCneViDE4mj0KYJavxJfAKXedCJKDiiIwWR2MLCEFciEIAIl6yJROMs+DObcn/KhcQVyD2FWfQsl+IiBoYv5rBx9IMPrZm8LE1g4+tGXxszeBjawYfWzP42JrBx9YMPrZm8LE1g4+tGXxszeBjawYfWzP4QbiEEAAhOsdKJ7gUHNpJvJD0i4MALsoyTA58a6QzkVdb1kTGjakcnXjQoRi+HBmicDIACJgcuIFKBFfFKMrSPF7d7bLSyqri9emujlVFzPgDzOTAeTToIQwq+Xi/Uzoht1aq8Ella6+XAUQIIsTpWZxBCA4MrG6t9l7ZQutCIbnyXhdFoW4bF6utcZMDL5tq21TxKadkLZJ7rbUn9qKw6DEqp1vhopGTAw/bEMtlzklZRNaqNXfRWt6npHNaq7wsq+8+zRA+vo1vaGbk9mjy1toFvQKNfkN/80oXz810fNyUhoOsgpDhxeeUi1NSCtF3j0w6AYH19vUfA8dMU5rgArzvvN3ndIob34x9YdPuxpnAt/2jy4+BS1lK5sI25rXer9VJg2ubPYKn9bvjsnG9N+mPgQfHDS9584q70hf+JLny5Ok66zUm1l/y/ODAQhWkeEI/sTYh+wnZ5P+EGX8dKgH9XfOnwKvGceHuMd5Z5bM/uTkpPhYUbrRaRBnOvzkhVLFc5SJl7wu9tyc3p8XAiI/Q631+57+as4Nj+AZxi2WJxmCo8klXwfKlyIUu8GH+Ncjzuwrdp6upnPorNPx6T0l/93a3uL97srrNPlYXvvV0m9D4tetdJf5c5sSEcut1B+5xF2abnldRtJU4r18x8WT0cEqi9CJwf77C+X2chXKF1ckxeqAv2/VjwDdCUBfkGtho6xMWiVi3WPIpjIyrCbhKgAVuyY6cisNbbNUwUmPzgHFPGAg7TXULpf2EGwEj/rM4P7iUmOuPrpK1vX78ZZzDdbEcAKzRS/64W1usVVKBvo+b0+r1+S3O4LHdfp3F7a6s4hbLQEEdJ8NO00gwSGxxeyqbclsprs4OLviStlxncaUfHp2MFS5J10nxh3G8alZYfmE/gX6OLQaW6vdnB+fhVuuPu3MttpwHIdC5JW1Qh79GkE+Z9iVugD3GnOzfzg4e4BpLPzT0X6n7wF3VfGl1Vugq+MZQT0e56uXs4ELkpAvbWdyuKhHQgT4/zuw05SBqn9HZdT47OGOecqE/WDzzSggsGD8/qnxrTU2bmMoDf3ZwURYZMzr1xq1eIYgqfB38wD2+L4o2MVblmKTODh5YUkXqXFwvRDBOmvLz4+QSjW3bWQuFleLs4BVghG5T+R8tsE3AuP21FnnIbdBsyxqMi2cHlxErPlV05exDLClyf2nN4JlipqbBVltKnh1cuKfd9cvL9UErHrgR5Rdw/oq9Ms1Y0OZYt6izg/9T5PDShYi9nU0YUMitsOPou9xo4JT9gwR4oOEtpZ8/CWjXd73xwDH3uxD4LtFMkeIhthranj/l//OJXACs2dHgOWHi/GN2lZa91xsL3NHZj2uIONGMwlP6tOn8Xf4/nyiUcRueEkZwmlBg26aSxRKy93pjgZtSNNVinxG8oGG5UhhV/O9LADeubokpX9KYX6Hhmwm0bidktrEMoTLY6DdrzDxdh0RnQrdw/qHn6WdoyhJYhOie0Ek6cKyA134lJjBXOSHBDXb4EoK7Lz40djSI1s98AtPaU6LjccmNg3elst0fp7g57YKZwHz85BMIFgRIKXY0vEjd+ELn4oZt+x9iDW9xA1zIEu5o/qxz5yzeP5QBvvak/1XDRxXmwARXY5ehaeLfRRXs7SB+7Un/q4aPKmBwdzbZaoRWx7F5boRkVZhuAjJOGGeeOmJsSy3+UJtQYavhpgsuDAP4fTR13tMpUboPHKQhT+qpwcExN8I7mrkbFCncpcU12ttImof2XXdwcEw9cqc/nlRgNWsfhTDYDkk53XAog3tO69SBUzDPS9be1WL06fqeGt5VXJ0+nuxjH6Fut9Fx3tp8uuAs5nbY1oEX9gUbZ8m5DILOK3pqePDrtkXrLK6LfAPMCPQUTKlierUKx3zuZCgf6HoT+go2yZqGdIW+imEbpne16aBSMmMwqa/3dM0J4W1CYyebngV2FXJ6l8kOEhI4br6F95kSJaVLbb1OL6sSQyHrvSkPGgy8PZndxmzpnkehdMa8Q6OrmyrECvofhR80GLjDbB5hk2jsg+yW7iYg/QMWXeBE/17zoOHAjSlleKbBD40jVNtEpLfAo3DBTBgcK6jKybVqBym5HWBllRtZ4nNIqL5/TXCwzQk8yNJrnWgS7q2iyzUPmOUdw9zTv7g6aLg4zgWwm2OqJ5OvQ/ntTdmtPxa40rp4w47/x9YfFHxzBKfbBw/u+y7SrT8YOHq5+ABOx+UbFr4cH35j/UHBu85HW+vfxUWAy0/gyjffT5gf1h+qrBWSi83HaaEuZGmm7+NfwHFzCt7/ZudnDQZusCsTm+M1ECwRC2n6z1E+a7iUTz3lho4e/l680VaD/H6NctBg4MAh8M1xOqswqpT0XD+k4cClC+XNERyL8uSk+clwO1CRFVrwYzjEUgscTN/iWAYa2OTO4tRJNEzGn1p+OFcBJ/lqcbf4q993i3vhqulbHIOhce54Pxyb54DbdfrhcGjN4GNrBh9bM/jYmsG/LIxtPn/sEufi7n7xXH3jI2Jf1h9sWsu4NPXxsgRWtdaU/U8gPms4VymdcBudDo0EgitW9T9l+6zhWjcuo7ux9nC7mW5oBxOnX49L4AE2/liPK6zHSz59HweoBHzsOW2hhQzTH3qyCNgsq04evVya/h/K+6zhTiRi5fjmeGvC+kIx/v1/lnHQgF2+kLw+fmaC/rsABDP9DijyirnNMarQpySZ6392/1kDZk5oxE3yR5P7HNkFtG7ClGa7+vChPQyI2IFOP3OCLIOoj/+Dgny8NHL683EIUkBddJ/E8oVOTva/9fZZw7mK4JXbrPddreJTjtUFnLrJEqRxH+5cQRnc9297dJobibE1g4+tGXxszeBjawYfWzP42JrBx9YMPrZm8LE1g4+tGXxszeBjawYfWzP42JrBx9YMPrY68Iv9guiL/Uruy/0S9Iv92nkJlYSLkqxA1qxuIki4HJNjTJEQm5qt0FEkoOtciAQHic6yYjerul5eXZSWdb26+R9eMbLIll+tqgAAAABJRU5ErkJggg=="
const flagLink = "https://upload.wikimedia.org/wikipedia/en/thumb/4/4c/Flag_of_Sweden.svg/800px-Flag_of_Sweden.svg.png"

export const aggPointsLeaderboard = [
    {
        id: 1,
        playerName: "Zyntex",
        aggPoints: 20859,
        avatar: avatarLink,
        nationality: flagLink,
    },
    {
        id: 2,
        playerName: "unity",
        aggPoints: 20721,
        avatar: avatarLink,
        nationality: flagLink,
    },
    {
        id: 3,
        playerName: "RealCreative",
        aggPoints: 20694,
        avatar: avatarLink,
        nationality: flagLink,
    },
]

export const aggTimeLeaderboard = [
    {
        id: 1,
        playerName: "unity",
        aggPoints: "53:18.51",
        avatar: avatarLink,
        nationality: flagLink,
    },
    {
        id: 2,
        playerName: "Zyntex",
        aggPoints: "53:25.90",
        avatar: avatarLink,
        nationality: flagLink,
    },
    {
        id: 3,
        playerName: "RealCreative",
        aggPoints: "53:38.68",
        avatar: avatarLink,
        nationality: flagLink,
    },
]