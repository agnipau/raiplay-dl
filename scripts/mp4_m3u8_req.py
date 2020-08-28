from requests import Request, Session


def make_request(mp4: bool = True):
    s = Session()
    req = Request(
        'GET', 'http://mediapolisvod.rai.it/relinker/relinkerServlet.htm?cont=mlvVsVxXVmco7ouKF7CFTgeeqqEEqualeeqqEEqual')
    prepped = req.prepare()
    prepped.headers['accept'] = '*/*'
    prepped.headers['host'] = 'mediapolisvod.rai.it'
    # Se includo l'header User-Agent il server mi risponde con un m3u8,
    # altrimenti con un mp4. Strano ma buono a sapersi
    if not mp4:
        prepped.headers['user-agent'] = 'mediapolisvod.rai.it'
    response = s.send(prepped, allow_redirects=False)
    print(response.headers['location'])


def main():
    make_request()
    make_request(False)


if __name__ == '__main__':
    main()
