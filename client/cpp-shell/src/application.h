#pragma once

#include <QObject>
#include <QQmlApplicationEngine>

class Application : public QObject {
    Q_OBJECT

public:
    explicit Application(QObject* parent = nullptr);
    ~Application();

private:
    QQmlApplicationEngine* m_engine;
};