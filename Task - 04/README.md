![ezgif-5-792c2494b7](https://user-images.githubusercontent.com/115163471/201459646-3c4dc427-7ea1-4735-8a30-75d24021db8c.gif)

# Python script for the bot
this was a complex task for me at the time of me doing this. I remember struggling with the csv part but i was able to complete it nevertheless.


    import os
    import telebot
    import requests
    import json
    import csv

    #TODO: 1.1 Get your environment variables
    yourkey = os.getenv("84ee025e")
    bot_id = os.getenv("epicbot")

    bot = telebot.TeleBot(bot_id)

    @bot.message_handler(commands=['start', 'hello'])
    def greet(message):
        global botRunning
        botRunning = True
        bot.reply_to(
            message, 'Hello there! I am a bot that will show movie information for you and export it in a CSV file.\n\n')
    
    @bot.message_handler(commands=['stop', 'bye'])
    def goodbye(message):
        global botRunning
        botRunning = False
        bot.reply_to(message, 'Bye!\nHave a good time')
    


    @bot.message_handler(func=lambda message: botRunning, commands=['help'])
    def helpProvider(message):
    bot.reply_to(message, '1.0 You can use \"/movie MOVIE_NAME\" command to get the details of a particular movie. For eg: \"/movie The Shawshank Redemption\"\n\n2.0. You can use \"/export\" command to export all the movie data in CSV format.\n\n3.0. You can use \"/stop\" or the command \"/bye\" to stop the bot.')


    @bot.message_handler(func=lambda message: botRunning, commands=['movie'])
    def getMovie(message):
        bot.reply_to(message, 'Getting movie info...')
    
    # TODO: 1.2 Get movie information from the API
    MOVIE_NAME=message.text
    MOVIE_NAME=str(MOVIE_NAME.split(' ',1)[1])
    response = requests.get(f"http://www.omdbapi.com/?apikey=84ee025e&t={MOVIE_NAME}")
    mv_data=response.json()
            
    # TODO: 1.3 Show the movie information in the chat window
    info = []
    for key, value in mv_data.items():
          if key != "Ratings":
              print(key, ' : ', value)
              sam = key +' : '+value
              info.append(sam)
   
    # TODO: 2.1 Create a CSV file and dump the movie information in it
              
    bot.reply_to(message, "\n".join(info))
    with open('movie_data_by_epicbot.csv', 'w',encoding ='UTF8') as csvfile:
            cswriter = csv.writer(csvfile)
            cswriter.writerow(info)

    #TODO: 2.2 Send downlodable CSV file to telegram chat
    @bot.message_handler(func=lambda message: botRunning, commands=['export'])
    def getList(message):
        bot.reply_to(message, 'Generating file.......')
        try:
           chat_id = message.chat.id
      
    except exception as a:
       print(a)
    mv_csv=open('movie_data_by_epicbot.csv','rb')
    
    bot.send_document(chat_id,mv_csv)
   

    @bot.message_handler(func=lambda message: botRunning)
    def default(message):
        bot.reply_to(message, 'I did not understand '+'\N{confused face}')
    
    bot.infinity_polling()
